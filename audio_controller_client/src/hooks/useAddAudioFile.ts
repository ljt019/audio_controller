import { useMutation, useQueryClient } from "@tanstack/react-query";
import { axiosInstance } from "@/hooks/axiosInstance";

async function addAudioFile(file: File) {
  const formData = new FormData();
  formData.append("file", file);

  const response = await axiosInstance.post(
    `/receive_audio_file?file_name=${encodeURIComponent(file.name)}`,
    formData,
    {
      headers: {
        "Content-Type": "multipart/form-data",
      },
    }
  );

  return response.data;
}

export function useAddAudioFile() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: addAudioFile,
    onSuccess: () => {
      queryClient.invalidateQueries({
        queryKey: ["audio-files"],
        refetchType: "active",
      });
    },
  });
}
