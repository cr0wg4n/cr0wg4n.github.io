// CloudFront Function (viewer request, runtime cloudfront-js-2.0).
// Sends www.cr0wg4n.com to cr0wg4n.com with a permanent redirect (301), keeping path and query string,
// so the site has a single host and search engines do not see two copies.
// Setup steps are in the README ("Deploy" section, www redirect).
function handler(event) {
  var request = event.request;
  var host = request.headers.host ? request.headers.host.value : '';

  if (host === 'www.cr0wg4n.com') {
    var parts = [];
    var query = request.querystring || {};
    Object.keys(query).forEach(function (key) {
      var item = query[key];
      if (item.multiValue) {
        item.multiValue.forEach(function (v) { parts.push(key + '=' + v.value); });
      } else {
        parts.push(item.value === '' ? key : key + '=' + item.value);
      }
    });
    var qs = parts.length ? '?' + parts.join('&') : '';

    return {
      statusCode: 301,
      statusDescription: 'Moved Permanently',
      headers: { location: { value: 'https://cr0wg4n.com' + request.uri + qs } },
    };
  }

  return request;
}
