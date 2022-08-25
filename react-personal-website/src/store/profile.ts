import { createSlice } from '@reduxjs/toolkit';
import { RootState } from '../app/store';
import data from '../data/profile';

export interface Profile {
  name: string,
  role: string,
  description: string,
  avatar: string,
  avatarHover?: string,
  motto?: string
}

const initialState: Profile = {
  ...data()
};

export const profileSlice = createSlice({
  name:'profile',
  initialState,
  reducers:{}
})

export const selectProfile = (state: RootState) => state.profile

// actions to dispatch
// export const {} = profileSlice.actions;

export default profileSlice.reducer;