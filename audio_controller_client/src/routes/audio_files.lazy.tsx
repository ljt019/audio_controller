import { createLazyFileRoute } from "@tanstack/react-router";
import {
  Table,
  TableCaption,
  TableHeader,
  TableRow,
  TableHead,
  TableBody,
  TableCell,
} from "@/components/ui/table";
import { Button } from "@/components/ui/button";
import useGetAudioFiles from "@/hooks/useGetAudioFiles";
import usePlayAudioFile from "@/hooks/usePlayAudioFile";
import useDeleteAudioFile from "@/hooks/useDeleteAudioFile";

export const Route = createLazyFileRoute("/audio_files")({
  component: AudioFilePage,
});

function AudioFilePage() {
  const { data: audioFiles, isLoading, isError } = useGetAudioFiles();

  if (isLoading) return <div>Loading...</div>;

  if (isError || !audioFiles) return <div>Error loading audio files.</div>;

  return (
    <Table>
      <TableCaption>
        A list of the audio files stored on the server.
      </TableCaption>
      <TableHeader>
        <TableRow>
          <TableHead>File Name</TableHead>
        </TableRow>
      </TableHeader>
      <TableBody>
        {audioFiles.map((audioFile: string) => (
          <TableRow key={audioFile}>
            <TableCell className="font-medium">{audioFile}</TableCell>
            <TableCell>
              <PlayButton audioFile={audioFile} />
            </TableCell>
            <TableCell>
              <RemoveButton audioFile={audioFile} />
            </TableCell>
          </TableRow>
        ))}
      </TableBody>
    </Table>
  );
}

function PlayButton({ audioFile }: { audioFile: string }) {
  const mutate = usePlayAudioFile();

  return (
    <Button
      variant={"ghost"}
      className="rounded-[0.5rem] text-muted-foreground hover:text-foreground"
      onClick={() => mutate.mutate(audioFile)}
    >
      Play
    </Button>
  );
}

function RemoveButton({ audioFile }: { audioFile: string }) {
  const mutate = useDeleteAudioFile();

  return (
    <Button
      variant={"ghost"}
      className="rounded-[0.5rem] text-muted-foreground hover:text-red-500"
      onClick={() => mutate.mutate(audioFile)}
    >
      Remove
    </Button>
  );
}
