import { createLazyFileRoute } from "@tanstack/react-router";

export const Route = createLazyFileRoute("/schedule")({
  component: Schedule,
});

function Schedule() {
  return (
    <div className="p-2">
      <h3>Welcome Home schedule!</h3>
    </div>
  );
}
