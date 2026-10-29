#!/usr/bin/env bash
# Removes what was created for the GitHub Actions -> AWS deploy:
#   - the IAM role github-actions-hugo-deploy (and its inline policy)
#   - the GitHub OIDC identity provider, unless another role still trusts it
# It does NOT touch the S3 bucket, CloudFront or Route 53.
#
# For a role created by docs/aws-oidc-deploy.yml, delete the CloudFormation stack instead; use this script for
# the one created by hand with the CLI.
#
#   AWS_PROFILE=cr0wg4n ./aws-oidc-teardown.sh --dry-run        # show what would be deleted
#   AWS_PROFILE=cr0wg4n ./aws-oidc-teardown.sh                  # delete (asks for the account id first)
#   AWS_PROFILE=cr0wg4n ./aws-oidc-teardown.sh --keep-provider  # delete only the role
set -euo pipefail

ROLE=github-actions-hugo-deploy
HOST=token.actions.githubusercontent.com
DRY=0; KEEP=0
for a in "$@"; do
  case $a in
    --dry-run) DRY=1 ;;
    --keep-provider) KEEP=1 ;;
    *) echo "unknown option: $a" >&2; exit 2 ;;
  esac
done
run() { if [ $DRY = 1 ]; then echo "[dry-run] $*"; else "$@"; fi; }

ACCOUNT=$(aws sts get-caller-identity --query Account --output text)
PROVIDER="arn:aws:iam::$ACCOUNT:oidc-provider/$HOST"
echo "Account $ACCOUNT"
if [ $DRY = 0 ]; then
  read -r -p "Type the account id to confirm the deletion: " confirm
  [ "$confirm" = "$ACCOUNT" ] || { echo "aborted"; exit 1; }
fi

if aws iam get-role --role-name "$ROLE" >/dev/null 2>&1; then
  for p in $(aws iam list-role-policies --role-name "$ROLE" --query PolicyNames --output text); do
    run aws iam delete-role-policy --role-name "$ROLE" --policy-name "$p"
  done
  for arn in $(aws iam list-attached-role-policies --role-name "$ROLE" --query 'AttachedPolicies[].PolicyArn' --output text); do
    run aws iam detach-role-policy --role-name "$ROLE" --policy-arn "$arn"
  done
  run aws iam delete-role --role-name "$ROLE"
else
  echo "role $ROLE not found, skipping"
fi

# The provider is shared by the whole account: keep it if any other role trusts GitHub.
if [ $KEEP = 0 ] && aws iam get-open-id-connect-provider --open-id-connect-provider-arn "$PROVIDER" >/dev/null 2>&1; then
  in_use=0
  for r in $(aws iam list-roles --query 'Roles[].RoleName' --output text); do
    [ "$r" = "$ROLE" ] && continue
    n=$(aws iam get-role --role-name "$r" --query "Role.AssumeRolePolicyDocument.Statement[?Principal.Federated=='$PROVIDER'] | length(@)" --output text)
    if [ "$n" != "0" ]; then echo "provider still trusted by role $r: keeping it"; in_use=1; fi
  done
  [ $in_use = 0 ] && run aws iam delete-open-id-connect-provider --open-id-connect-provider-arn "$PROVIDER"
fi
echo "done"
