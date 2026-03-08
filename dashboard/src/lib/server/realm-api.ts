import { redirect } from '@sveltejs/kit';
import type { RequestEvent } from '@sveltejs/kit';
import { apiRequest, ApiError } from '$lib/api/client';

export async function loadRealmResource<T>(
  event: Pick<RequestEvent, 'cookies' | 'fetch' | 'params' | 'url'>,
  path: string
) {
  const token = event.cookies.get('FERRISKEY_IDENTITY');

  try {
    return await apiRequest<T>({
      url: event.url,
      fetcher: event.fetch,
      path,
      init: {
        headers: {
          Authorization: `Bearer ${token}`
        }
      }
    });
  } catch (error) {
    if (error instanceof ApiError && error.status === 401) {
      throw redirect(303, `/realms/${event.params.realm}/authentication/login`);
    }

    throw error;
  }
}
