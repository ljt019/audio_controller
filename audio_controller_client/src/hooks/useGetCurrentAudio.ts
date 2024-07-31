import { useQuery } from "@tanstack/react-query";
import { axiosInstance } from "@/hooks/axiosInstance";

async function getCurrentAudio() {
  const { data } = await axiosInstance.get("/get_current_audio");

  // split "audio_files\" off of data string

  const split_data = data.split("audio_files\\")[1];

  return split_data;
}

export default function useGetCurrentAudio() {
  return useQuery({
    queryKey: ["current-audio"],
    queryFn: getCurrentAudio,
  });
}
