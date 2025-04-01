import cv2
import requests
import io
import numpy as np
from pipe import Lava

def test_img(url):

    try:
        response = requests.get(url, stream=True)
        response.raise_for_status()  # Raise HTTPError for bad responses (4xx or 5xx)

        bytes_io = io.BytesIO()
        for chunk in response.iter_content(chunk_size=1024 * 1024):  # Chunk size in MB
            bytes_io.write(chunk)
        bytes_io.seek(0)  # Reset the file pointer to the beginning

        bytes_array = np.asarray(bytearray(bytes_io.read()), dtype=np.uint8)
        cap = cv2.imdecode(bytes_array, cv2.IMREAD_COLOR)

        if cap is None:
            print("Could not decode image")
            exit()

        cv2.imshow('Frame', cap)
        cv2.waitKey(0)

    except requests.exceptions.RequestException as e:
        print(f"Error fetching URL: {e}")

    cv2.destroyAllWindows()

def run_vid(url, model):
    output = "NULL"
    frame_count = 0
    cap = cv2.VideoCapture(url)

    if not cap.isOpened():
        print("Error: Could not open video stream.")
        exit()

    while True:
        ret, frame = cap.read()
        if not ret:
            break

        output += model.inference(image=frame)

        cv2.imshow('Frame', frame)
        frame_count += 1
        print(f"Frame count {frame_count:04d}")

        if cv2.waitKey(1) & 0xFF == ord('q'):
            break

    cap.release()
    cv2.destroyAllWindows()
    return output