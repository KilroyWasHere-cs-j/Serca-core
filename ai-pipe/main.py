from transformers import BitsAndBytesConfig, LlavaNextVideoForConditionalGeneration, LlavaNextVideoProcessor
import torch
import cv2
import numpy as np

# Quantization config for the model
quantization_config = BitsAndBytesConfig(
    load_in_4bit=True,
    bnb_4bit_compute_dtype=torch.float16
)

# Load processor and model
processor = LlavaNextVideoProcessor.from_pretrained("llava-hf/LLaVA-NeXT-Video-7B-hf")
model = LlavaNextVideoForConditionalGeneration.from_pretrained(
    "llava-hf/LLaVA-NeXT-Video-7B-hf",
    quantization_config=quantization_config,
    device_map='auto'
)

def stream_video_opencv(url):
    '''
    Stream the video from a URL using OpenCV and return the capture object.
    
    Args:
        url (str): URL of the video to stream.
    
    Returns:
        cv2.VideoCapture: OpenCV capture object for streaming video.
    '''
    capture = cv2.VideoCapture(url)
    if not capture.isOpened():
        raise ValueError(f"Error opening video stream from {url}")
    return capture

def read_video_opencv(capture, indices):
    '''
    Decode the video using OpenCV and extract frames based on given indices.
    
    Args:
        capture (cv2.VideoCapture): OpenCV video capture object.
        indices (List[int]): List of frame indices to decode.
    
    Returns:
        np.ndarray: np array of decoded frames of shape (num_frames, height, width, 3).
    '''
    frames = []
    total_frames = int(capture.get(cv2.CAP_PROP_FRAME_COUNT))  # Total number of frames in the video
    
    for idx in indices:
        capture.set(cv2.CAP_PROP_POS_FRAMES, idx)  # Set the capture position to the frame index
        ret, frame = capture.read()
        if ret:
            frames.append(frame)
    return np.array(frames)

# Video URL (replace with your desired video URL)
video_url = "http://dash.edgesuite.net/akamai/mp4/hdworld_640x360_700_b.mp4"  # Replace with the actual video URL

# Stream the video from the URL using OpenCV
capture = stream_video_opencv(video_url)

# Get total frames and sample uniformly 8 frames
total_frames = int(capture.get(cv2.CAP_PROP_FRAME_COUNT))
indices = np.arange(0, total_frames, total_frames / 8).astype(int)

# Read sampled frames from the video
clip = read_video_opencv(capture, indices)

# Release the capture object after use
capture.release()

# Conversation with the assistant
conversation = [
  {
          "role": "user",
          "content": [
              {"type": "text", "text": "What do you see in this video?"},
              {"type": "video"},
              ],
      },
      {
          "role": "assistant",
          "content": [
              {"type": "text", "text": "I see a baby reading a book."},
              ],
      },
      {
          "role": "user",
          "content": [
              {"type": "text", "text": "Why is it funny?"},
              ],
      },
]

# Generate the prompt
prompt = processor.apply_chat_template(conversation, add_generation_prompt=True)

# Prepare inputs for the model
inputs = processor([prompt], videos=[clip], padding=True, return_tensors="pt").to(model.device)

# Generate output from the model
generate_kwargs = {"max_new_tokens": 100, "do_sample": True, "top_p": 0.9}
output = model.generate(**inputs, **generate_kwargs)

# Decode the output
generated_text = processor.batch_decode(output, skip_special_tokens=True)

# Print the generated response
print(generated_text)
