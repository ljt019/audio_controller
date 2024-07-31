import { useMutation } from "@tanstack/react-query";
import { axiosInstance } from "@/hooks/axiosInstance";

async function playAudioFile(audioFile: string) {
  const { data } = await axiosInstance.post(
    `/play_audio?file_name=${audioFile}`
  );
  return data;
}

export default function usePlayAudioFile() {
  return useMutation({
    mutationFn: playAudioFile,
  });
}
