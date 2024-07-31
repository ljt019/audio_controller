/* prettier-ignore-start */

/* eslint-disable */

// @ts-nocheck

// noinspection JSUnusedGlobalSymbols

// This file is auto-generated by TanStack Router

import { createFileRoute } from '@tanstack/react-router'

// Import Routes

import { Route as rootRoute } from './routes/__root'

// Create Virtual Routes

const ScheduleLazyImport = createFileRoute('/schedule')()
const DashboardLazyImport = createFileRoute('/dashboard')()
const AudiofilesLazyImport = createFileRoute('/audio_files')()

// Create/Update Routes

const ScheduleLazyRoute = ScheduleLazyImport.update({
  path: '/schedule',
  getParentRoute: () => rootRoute,
} as any).lazy(() => import('./routes/schedule.lazy').then((d) => d.Route))

const DashboardLazyRoute = DashboardLazyImport.update({
  path: '/dashboard',
  getParentRoute: () => rootRoute,
} as any).lazy(() => import('./routes/dashboard.lazy').then((d) => d.Route))

const AudiofilesLazyRoute = AudiofilesLazyImport.update({
  path: '/audio_files',
  getParentRoute: () => rootRoute,
} as any).lazy(() => import('./routes/audio_files.lazy').then((d) => d.Route))

// Populate the FileRoutesByPath interface

declare module '@tanstack/react-router' {
  interface FileRoutesByPath {
    '/audio_files': {
      id: '/audio_files'
      path: '/audio_files'
      fullPath: '/audio_files'
      preLoaderRoute: typeof AudiofilesLazyImport
      parentRoute: typeof rootRoute
    }
    '/dashboard': {
      id: '/dashboard'
      path: '/dashboard'
      fullPath: '/dashboard'
      preLoaderRoute: typeof DashboardLazyImport
      parentRoute: typeof rootRoute
    }
    '/schedule': {
      id: '/schedule'
      path: '/schedule'
      fullPath: '/schedule'
      preLoaderRoute: typeof ScheduleLazyImport
      parentRoute: typeof rootRoute
    }
  }
}

// Create and export the route tree

export const routeTree = rootRoute.addChildren({
  AudiofilesLazyRoute,
  DashboardLazyRoute,
  ScheduleLazyRoute,
})

/* prettier-ignore-end */

/* ROUTE_MANIFEST_START
{
  "routes": {
    "__root__": {
      "filePath": "__root.tsx",
      "children": [
        "/audio_files",
        "/dashboard",
        "/schedule"
      ]
    },
    "/audio_files": {
      "filePath": "audio_files.lazy.tsx"
    },
    "/dashboard": {
      "filePath": "dashboard.lazy.tsx"
    },
    "/schedule": {
      "filePath": "schedule.lazy.tsx"
    }
  }
}
ROUTE_MANIFEST_END */
