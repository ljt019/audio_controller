import { useQuery } from "@tanstack/react-query";
import { axiosInstance } from "@/hooks/axiosInstance";

async function getAudioFiles() {
  const { data } = await axiosInstance.get("/get_audio_files");
  return data;
}

export default function useGetAudioFiles() {
  return useQuery({
    queryKey: ["audio-files"],
    queryFn: getAudioFiles,
  });
}
