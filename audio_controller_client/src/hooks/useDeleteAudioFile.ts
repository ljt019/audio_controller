import { useMutation, useQueryClient } from "@tanstack/react-query";
import { axiosInstance } from "@/hooks/axiosInstance";

async function deleteAudioFile(audioFile: string) {
  const { data } = await axiosInstance.delete(
    `/delete_audio_file?file_name=${audioFile}`
  );
  return data;
}

export default function useDeleteAudioFile() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: deleteAudioFile,
    onSuccess: () => {
      queryClient.invalidateQueries({
        queryKey: ["audio-files"],
        refetchType: "active",
      });
    },
  });
}
