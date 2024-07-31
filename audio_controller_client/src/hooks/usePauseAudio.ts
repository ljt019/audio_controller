import { useMutation } from "@tanstack/react-query";
import { axiosInstance } from "@/hooks/axiosInstance";

async function pauseAudioFile() {
  const { data } = await axiosInstance.post(`/pause_audio`);
  return data;
}

export default function usePauseAudioFile() {
  return useMutation({
    mutationFn: pauseAudioFile,
  });
}
