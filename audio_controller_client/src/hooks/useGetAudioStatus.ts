import { useQuery } from "@tanstack/react-query";
import { axiosInstance } from "@/hooks/axiosInstance";

async function getAudioStatus() {
  const { data } = await axiosInstance.get("/get_audio_status");
  return data;
}

export default function useGetAudioStatus() {
  return useQuery({
    queryKey: ["audio-status"],
    queryFn: getAudioStatus,
  });
}
