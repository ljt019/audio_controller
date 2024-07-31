import { useQuery } from "@tanstack/react-query";
import { axiosInstance } from "@/hooks/axiosInstance";

async function getVolume() {
  const { data } = await axiosInstance.get("/get_volume");
  return data;
}

export default function useGetVolume() {
  return useQuery({
    queryKey: ["volume"],
    queryFn: getVolume,
  });
}
