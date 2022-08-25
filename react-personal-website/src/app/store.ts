import { configureStore, ThunkAction, Action } from '@reduxjs/toolkit';
import profileReducer from '../store/profile';


export const store = configureStore({
  reducer: {
    profile: profileReducer
  },
});

export type AppDispatch = typeof store.dispatch;
export type RootState = ReturnType<typeof store.getState>;
export type AppThunk<ReturnType = void> = ThunkAction<
  ReturnType,
  RootState,
  unknown,
  Action<string>
>;
