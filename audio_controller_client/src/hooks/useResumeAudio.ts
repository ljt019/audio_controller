import { useMutation } from "@tanstack/react-query";
import { axiosInstance } from "@/hooks/axiosInstance";

async function resumeAudioFile() {
  const { data } = await axiosInstance.post(`/resume_audio`);
  return data;
}

export default function useResumeAudioFile() {
  return useMutation({
    mutationFn: resumeAudioFile,
  });
}
