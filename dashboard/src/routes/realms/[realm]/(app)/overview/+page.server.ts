import type { PageServerLoad } from './$types';
import { loadRealmResource } from '$lib/server/realm-api';

type UserRole = {
  id: string;
  name: string;
};

type User = {
  client_id?: string | null;
  email_verified: boolean;
  enabled: boolean;
  id: string;
  required_actions: string[];
  roles?: UserRole[] | null;
};

type Client = {
  client_id: string;
  direct_access_grants_enabled: boolean;
  enabled: boolean;
  id: string;
  public_client: boolean;
  redirect_uris?: Array<{ uri: string }> | null;
  secret?: string | null;
  service_account_enabled: boolean;
};

type Role = {
  client_id?: string | null;
  id: string;
  permissions: string[];
};

type UsersResponse = { data: User[] };
type ClientsResponse = { data: Client[] };
type RolesResponse = { data: Role[] };

export const load: PageServerLoad = async ({ cookies, fetch, params, url }) => {
  const [usersResponse, clientsResponse, rolesResponse] = await Promise.all([
    loadRealmResource<UsersResponse>(
      { cookies, fetch, params, url },
      `/realms/${params.realm}/users`
    ),
    loadRealmResource<ClientsResponse>(
      { cookies, fetch, params, url },
      `/realms/${params.realm}/clients`
    ),
    loadRealmResource<RolesResponse>(
      { cookies, fetch, params, url },
      `/realms/${params.realm}/roles`
    )
  ]);

  return {
    users: usersResponse.data,
    clients: clientsResponse.data,
    roles: rolesResponse.data
  };
};
