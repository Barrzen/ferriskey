import type { PageServerLoad } from './$types';
import { loadRealmResource } from '$lib/server/realm-api';

type Role = {
  id: string;
  name: string;
};

type User = {
  client_id?: string | null;
  created_at: string;
  email: string;
  email_verified: boolean;
  enabled: boolean;
  firstname: string;
  id: string;
  lastname: string;
  required_actions: string[];
  roles?: Role[] | null;
  username: string;
};

type UsersResponse = {
  data: User[];
};

export const load: PageServerLoad = async ({ cookies, fetch, params, url }) => {
  const response = await loadRealmResource<UsersResponse>(
    { cookies, fetch, params, url },
    `/realms/${params.realm}/users`
  );

  return {
    users: response.data
  };
};
