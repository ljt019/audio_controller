import { createLazyFileRoute } from "@tanstack/react-router";

export const Route = createLazyFileRoute("/schedule")({
  component: Index,
});

function Index() {
  return (
    <div className="p-2">
      <h3>Welcome Home schedule!</h3>
    </div>
  );
}
