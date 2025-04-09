# main.py
# TODO Switch to ffmpeg
import utils
import pipe
from pipe import Lava
from colorama import Fore, Back, Style
from streamer import run_vid
import cv2


def main():

    image_path = "bagel.jpg"
    output = "NULL"
    prompt, model_id = utils.load()

    lava = pipe.Lava(model_id=model_id, prompt=prompt)
    lava.load_model()

    video_url = "https://a.nwps.fi/m2-res_662p.mp4" 
    # Initialize video stream
    cap = cv2.VideoCapture(video_url)

    frame_inc = 16
    frame_count = 0
    while True:
        # Capture frame-by-frame from the video
        ret, frame = cap.read()
    
        # If a frame is read correctly, ret will be True
        if not ret:
            print("Failed to grab frame")
            break

        if frame_count % frame_inc == 0:
             # Perform inference on the current frame
            output = lava.inference(frame)
            print(output)

            # Display the frame with inference result (you can adjust this to show other info)
            cv2.putText(frame, str(frame_count), (50, 50), cv2.FONT_HERSHEY_SIMPLEX, 1, (0, 255, 0), 2, cv2.LINE_AA)

            # Show the processed frame
            cv2.imshow("Video Streaming with Inference", frame)

        frame_count += 1

        # Press 'q' to exit
        if cv2.waitKey(1) & 0xFF == ord('q'):
            break

    # Release the video capture object and close any OpenCV windows
    cap.release()
    cv2.destroyAllWindows()
    
if __name__ == "__main__":
    main()