import { createLazyFileRoute } from "@tanstack/react-router";
import { AudioFileUploader } from "@/components/AudioFileUploader";
import { Card, CardContent } from "@/components/ui/card";
import useGetCurrentAudio from "@/hooks/useGetCurrentAudio";
import useGetAudioStatus from "@/hooks/useGetAudioStatus";

export const Route = createLazyFileRoute("/dashboard")({
  component: Dashboard,
});

function Dashboard() {
  return (
    <div>
      <CurrentlyPlayingCard />
    </div>
  );
}

function CurrentlyPlayingCard() {
  const {
    data: currentAudio,
    isLoading: isCurrentAudioLoading,
    isError: isCurrentAudioError,
  } = useGetCurrentAudio();
  const {
    data: isPlaying,
    isLoading: isPlayingLoading,
    isError: isPlayingError,
  } = useGetAudioStatus();

  if (isCurrentAudioLoading || isPlayingLoading) {
    return <div>Loading...</div>;
  }

  if (isCurrentAudioError || isPlayingError || !currentAudio || !isPlaying) {
    return <div>Error...</div>;
  }

  // if audio is playing display Currently Playing
  // if audio is not playing display Last Played
  return (
    <Card>
      <CardContent>
        {isPlaying ? <div>Currently Playing</div> : <div>Last Played</div>}
        <div>{currentAudio}</div>
      </CardContent>
    </Card>
  );
}

function RecentlyPlayedCard() {
  return (
    <Card>
      <CardContent>
        <div>Recently Played</div>
      </CardContent>
    </Card>
  );
}

function UploadAudioFileCard() {
  return (
    <Card>
      <CardContent>
        <AudioFileUploader />
      </CardContent>
    </Card>
  );
}
