import { useQuery } from "@tanstack/react-query";
import axios from "axios";

export default function useGetAudioFiles() {
  return useQuery({
    queryKey: ["audio-files"],
    queryFn: async () => {
      const { data } = await axios.get("http://localhost:3030/get_audio_files");
      return data;
    },
  });
}
