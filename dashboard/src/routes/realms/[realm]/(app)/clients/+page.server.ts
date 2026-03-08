import type { PageServerLoad } from './$types';
import { loadRealmResource } from '$lib/server/realm-api';

type RedirectUri = {
  uri: string;
};

type Client = {
  client_id: string;
  client_type: string;
  created_at: string;
  direct_access_grants_enabled: boolean;
  enabled: boolean;
  id: string;
  name: string;
  protocol: string;
  public_client: boolean;
  redirect_uris?: RedirectUri[] | null;
  secret?: string | null;
  service_account_enabled: boolean;
  updated_at: string;
};

type ClientsResponse = {
  data: Client[];
};

export const load: PageServerLoad = async ({ cookies, fetch, params, url }) => {
  const response = await loadRealmResource<ClientsResponse>(
    { cookies, fetch, params, url },
    `/realms/${params.realm}/clients`
  );

  return {
    clients: response.data
  };
};
