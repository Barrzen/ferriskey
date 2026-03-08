import { fail, redirect } from '@sveltejs/kit';
import type { Actions, PageServerLoad } from './$types';
import { loadRealmResource } from '$lib/server/realm-api';
import { apiRequest, ApiError } from '$lib/api/client';

type Realm = {
  id: string;
  name: string;
  created_at: string;
  updated_at: string;
};

type RealmListResponse = {
  data: Realm[];
};

type UpdateRealmResponse = {
  data: Realm;
};

export const load: PageServerLoad = async ({ cookies, fetch, params, url }) => {
  const response = await loadRealmResource<RealmListResponse>(
    { cookies, fetch, params, url },
    `/realms/${params.realm}/users/@me/realms`
  );

  const realms = [...response.data].sort((left, right) => {
    if (left.name === params.realm) return -1;
    if (right.name === params.realm) return 1;
    return left.name.localeCompare(right.name);
  });

  return {
    realms
  };
};

export const actions: Actions = {
  create: async ({ cookies, fetch, params, request, url }) => {
    const form = await request.formData();
    const name = form.get('name')?.toString().trim() ?? '';

    if (!name) {
      return fail(400, { error: 'Realm name is required.' });
    }

    const token = cookies.get('FERRISKEY_IDENTITY');

    try {
      const realm = await apiRequest<Realm>({
        url,
        fetcher: fetch,
        path: '/realms',
        init: {
          method: 'POST',
          headers: { Authorization: `Bearer ${token}` },
          body: JSON.stringify({ name })
        }
      });

      return { success: true, message: `Realm "${realm.name}" created.` };
    } catch (error) {
      return fail(error instanceof ApiError ? error.status : 500, {
        error: error instanceof ApiError ? error.message : 'Failed to create realm.'
      });
    }
  },

  rename: async ({ cookies, fetch, params, request, url }) => {
    const form = await request.formData();
    const realmName = form.get('realm_name')?.toString().trim() ?? '';
    const name = form.get('name')?.toString().trim() ?? '';

    if (!realmName || !name) {
      return fail(400, { error: 'Current realm name and new name are required.' });
    }

    const token = cookies.get('FERRISKEY_IDENTITY');

    try {
      const response = await apiRequest<UpdateRealmResponse>({
        url,
        fetcher: fetch,
        path: `/realms/${realmName}`,
        init: {
          method: 'PUT',
          headers: { Authorization: `Bearer ${token}` },
          body: JSON.stringify({ name })
        }
      });

      if (realmName === params.realm) {
        throw redirect(303, `/realms/${response.data.name}/realms`);
      }

      return {
        success: true,
        message: `Realm "${realmName}" renamed to "${response.data.name}".`
      };
    } catch (error) {
      if (error instanceof Response) {
        throw error;
      }

      return fail(error instanceof ApiError ? error.status : 500, {
        error: error instanceof ApiError ? error.message : 'Failed to rename realm.'
      });
    }
  },

  delete: async ({ cookies, fetch, params, request, url }) => {
    const form = await request.formData();
    const realmName = form.get('realm_name')?.toString().trim() ?? '';

    if (!realmName) {
      return fail(400, { error: 'Realm name is required.' });
    }

    if (realmName === params.realm) {
      return fail(400, {
        error: 'Switch to another realm before deleting the current one.'
      });
    }

    const token = cookies.get('FERRISKEY_IDENTITY');

    try {
      await apiRequest<string>({
        url,
        fetcher: fetch,
        path: `/realms/${realmName}`,
        init: {
          method: 'DELETE',
          headers: { Authorization: `Bearer ${token}` }
        }
      });

      return { success: true, message: `Realm "${realmName}" deleted.` };
    } catch (error) {
      return fail(error instanceof ApiError ? error.status : 500, {
        error: error instanceof ApiError ? error.message : 'Failed to delete realm.'
      });
    }
  }
};
