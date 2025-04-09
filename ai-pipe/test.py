import ffmpeg
import cv2
import numpy as np

# Define the video stream URL
video_url = "https://a.nwps.fi/m2-res_662p.mp4"

# Use ffmpeg to read the stream and pipe it to OpenCV
process = ffmpeg.input(video_url).output('pipe:1', format='rawvideo', pix_fmt='bgr24').run_async(pipe_stdout=True, pipe_stderr=True)

while True:
    # Read one frame at a time from ffmpeg pipe
    in_bytes = process.stdout.read(640 * 480 * 3)  # Adjust to your frame size (width * height * 3 for RGB)
    if not in_bytes:
        break

    # Convert bytes to a numpy array for OpenCV
    frame = np.frombuffer(in_bytes, np.uint8).reshape([480, 640, 3])  # Adjust size to match your stream's resolution

    # Show the frame
    cv2.putText(frame, output, (50, 50), cv2.FONT_HERSHEY_SIMPLEX, 1, (0, 255, 0), 2, cv2.LINE_AA)
    cv2.imshow("Video Streaming with Inference", frame)

    # Exit condition
    if cv2.waitKey(1) & 0xFF == ord('q'):
        break

# Close the process and release OpenCV resources
process.stdout.close()
process.stderr.close()
process.wait()
cv2.destroyAllWindows()
