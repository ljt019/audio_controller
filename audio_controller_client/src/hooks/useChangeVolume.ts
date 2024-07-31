import { useMutation } from "@tanstack/react-query";
import { axiosInstance } from "@/hooks/axiosInstance";

// new_volume as float between 0.0 and 1.0
// error if new_volume is not in range
async function changeVolume(new_volume: number) {
  if (new_volume < 0.0 || new_volume > 1.0) {
    throw new Error("Volume must be between 0.0 and 1.0");
  }

  const response = await axiosInstance.post("/change_volume", new_volume);
  return response.data;
}

export default function useChangeVolume() {
  return useMutation({
    mutationFn: changeVolume,
    onError: (error) => {
      console.error("Failed to change volume:", error);
    },
  });
}
