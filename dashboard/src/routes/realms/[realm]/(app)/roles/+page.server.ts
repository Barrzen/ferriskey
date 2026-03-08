import type { PageServerLoad } from './$types';
import { loadRealmResource } from '$lib/server/realm-api';

type Role = {
  client_id?: string | null;
  created_at: string;
  description?: string | null;
  id: string;
  name: string;
  permissions: string[];
  updated_at: string;
};

type UserRole = {
  id: string;
  name: string;
};

type User = {
  id: string;
  roles?: UserRole[] | null;
};

type GetRolesResponse = {
  data: Role[];
};

type UsersResponse = {
  data: User[];
};

export const load: PageServerLoad = async ({ cookies, fetch, params, url }) => {
  const [rolesResponse, usersResponse] = await Promise.all([
    loadRealmResource<GetRolesResponse>(
      { cookies, fetch, params, url },
      `/realms/${params.realm}/roles`
    ),
    loadRealmResource<UsersResponse>(
      { cookies, fetch, params, url },
      `/realms/${params.realm}/users`
    )
  ]);

  return {
    roles: rolesResponse.data,
    users: usersResponse.data
  };
};
