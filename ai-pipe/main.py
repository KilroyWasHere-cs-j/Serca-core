# main.py
# TODO Switch to ffmpeg
from pipe import inference
from colorama import Fore, Back, Style
import cv2

path = "descriptions.txt"


def grab_frames(url, frame_snaps):
    video_url = url
    frame_inc = frame_snaps
    frame_count = 0
    frames = []
    # Initialize video stream
    cap = cv2.VideoCapture(video_url)
    print("Grabbing frames")
    print(video_url)
    while True:
        # Capture frame-by-frame from the video
        ret, frame = cap.read()
        # If a frame is read correctly, ret will be True
        if not ret:
            print("Failed to grab frame")
            break

        if frame_count % frame_inc == 0:
            frames.append(frame)
            # Display the frame with inference result (you can adjust this to show other info)
            cv2.putText(frame, str(frame_count), (50, 50), cv2.FONT_HERSHEY_SIMPLEX, 1, (0, 255, 0), 2, cv2.LINE_AA)
            # Show the processed frame
            cv2.imshow("Video Streaming with Inference", frame)
            inference()
        frame_count += 1

        if cv2.waitKey(1) & 0xFF == ord('q'):
            break

    print("Finished grabbing frames")
    # Release the video capture object
    cap.release()
    cv2.destroyAllWindows()
    print(len(frames))
    return frames

def push_into_file(url, description):
    file1 = open(path, "a")
    file1.write("\n")
    file1.write("--")
    file1.write(url)
    file1.write("\n")
    file1.write(description)
    file1.close()


def main():
    urls = []
    for url in urls:
        # Grab frames from the video at url
        frames = grab_frames(url, 15)
        description = process_frames(frames=frames)
        push_into_file(url=url, description=description)
    
if __name__ == "__main__":
    main()