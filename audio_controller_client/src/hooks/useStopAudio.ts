import { useMutation } from "@tanstack/react-query";
import { axiosInstance } from "@/hooks/axiosInstance";

async function playAudioFile() {
  const { data } = await axiosInstance.post(`/stop_audio`);
  return data;
}

export default function usePlayAudioFile() {
  return useMutation({
    mutationFn: playAudioFile,
  });
}
