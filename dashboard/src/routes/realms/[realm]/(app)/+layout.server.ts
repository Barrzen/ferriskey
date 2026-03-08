import { redirect } from '@sveltejs/kit';
import type { LayoutServerLoad } from './$types';
import { getSessionUser } from '$lib/auth/session';
import { loadRealmResource } from '$lib/server/realm-api';
import {
  hasTransientAuthParams,
  sanitizeCurrentAppPath
} from '$lib/utils/auth-redirect';

type RealmInfo = {
  id: string;
  name: string;
};

export const load: LayoutServerLoad = async ({ cookies, fetch, params, url }) => {
  const sessionUser = getSessionUser(cookies.get('FERRISKEY_IDENTITY'));
  const nextPath = sanitizeCurrentAppPath(url);

  if (!sessionUser) {
    throw redirect(
      303,
      `/realms/${params.realm}/authentication/login?next=${encodeURIComponent(nextPath)}`
    );
  }

  if (hasTransientAuthParams(url)) {
    throw redirect(303, nextPath || `/realms/${params.realm}/overview`);
  }

  let realms: RealmInfo[] = [];
  try {
    const res = await loadRealmResource<{ data: RealmInfo[] }>(
      { cookies, fetch, params, url },
      `/realms/${params.realm}/users/@me/realms`
    );
    realms = res.data;
  } catch {
    realms = [{ id: params.realm, name: params.realm }];
  }

  return {
    sessionUser,
    realms
  };
};
