import { createRootRoute, Link, Outlet } from "@tanstack/react-router";
import { TanStackRouterDevtools } from "@tanstack/router-devtools";
import { AudioLines, Gauge, CalendarClock } from "lucide-react";

export const Route = createRootRoute({
  component: () => (
    <div className="flex h-screen">
      <div className="w-[30%] sm:w-[20%] lg:w-[18%] xl:w-[15%] 2xl:w-[10%] p-4 flex flex-col gap-4 border-r border-white">
        <Link
          to="/dashboard"
          className="text-muted-foreground [&.active]:font-bold [&.active]:text-foreground"
        >
          <div className="flex gap-x-2">
            <Gauge /> Dashboard
          </div>
        </Link>
        <Link
          to="/audio_files"
          className="text-muted-foreground [&.active]:font-bold [&.active]:text-foreground"
        >
          <div className="flex gap-x-2">
            <AudioLines />
            Sounds
          </div>
        </Link>
        <Link
          to="/schedule"
          className="text-muted-foreground [&.active]:font-bold [&.active]:text-foreground"
        >
          <div className="flex gap-x-2">
            <CalendarClock />
            Schedule
          </div>
        </Link>
      </div>
      <div className="flex-1 p-4">
        <Outlet />
        <TanStackRouterDevtools />
      </div>
    </div>
  ),
});
