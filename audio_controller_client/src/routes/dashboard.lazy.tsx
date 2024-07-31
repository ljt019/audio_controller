import { createLazyFileRoute } from "@tanstack/react-router";
import { AudioFileUploader } from "@/components/AudioFileUploader";

export const Route = createLazyFileRoute("/dashboard")({
  component: About,
});

function About() {
  return (
    <div className="p-2">
      <AudioFileUploader />
    </div>
  );
}
