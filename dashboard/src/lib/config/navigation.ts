import {
  Activity,
  BadgeCheck,
  Compass,
  KeyRound,
  LayoutDashboard,
  Network,
  PanelsTopLeft,
  Shield,
  Users,
  Waypoints
} from 'lucide-svelte';

type IconComponent = typeof Activity;

export type NavigationItem = {
  title: string;
  description: string;
  icon: IconComponent;
  href: (realm: string) => string;
  tag?: string;
};

export type NavigationGroup = {
  title: string;
  items: NavigationItem[];
};

export const navigationGroups: NavigationGroup[] = [
  {
    title: 'Core',
    items: [
      {
        title: 'Overview',
        description: 'Live realm health and trends',
        icon: LayoutDashboard,
        href: (realm) => `/realms/${realm}/overview`
      },
      {
        title: 'Users',
        description: 'Lifecycle and credentials',
        icon: Users,
        href: (realm) => `/realms/${realm}/users`
      },
      {
        title: 'Clients',
        description: 'Applications and secrets',
        icon: PanelsTopLeft,
        href: (realm) => `/realms/${realm}/clients`
      },
      {
        title: 'Roles',
        description: 'Permissions and mapping',
        icon: BadgeCheck,
        href: (realm) => `/realms/${realm}/roles`
      }
    ]
  },
  {
    title: 'Configuration',
    items: [
      {
        title: 'Client Scopes',
        description: 'Claims and protocol mappers',
        icon: KeyRound,
        href: (realm) => `/realms/${realm}/client-scopes`
      },
      {
        title: 'Realm Settings',
        description: 'Branding, login, policies',
        icon: Shield,
        href: (realm) => `/realms/${realm}/realm-settings`
      },
      {
        title: 'Identity Providers',
        description: 'Social and enterprise SSO',
        icon: Network,
        href: (realm) => `/realms/${realm}/identity-providers`
      },
      {
        title: 'User Federation',
        description: 'LDAP and external stores',
        icon: Waypoints,
        href: (realm) => `/realms/${realm}/user-federation`
      }
    ]
  },
  {
    title: 'Security',
    items: [
      {
        title: 'SeaWatch',
        description: 'Audit stream and alerts',
        icon: Activity,
        href: (realm) => `/realms/${realm}/seawatch`,
        tag: 'Live'
      },
      {
        title: 'Compass',
        description: 'Authentication flow maps',
        icon: Compass,
        href: (realm) => `/realms/${realm}/compass`,
        tag: 'Beta'
      }
    ]
  }
];
