from transformers import BitsAndBytesConfig, LlavaNextVideoForConditionalGeneration, LlavaNextVideoProcessor
import torch

quantization_config = BitsAndBytesConfig(
    load_in_4bit=True,
    bnb_4bit_compute_dtype=torch.float16
)

processor = LlavaNextVideoProcessor.from_pretrained("llava-hf/LLaVA-NeXT-Video-7B-hf")
model = LlavaNextVideoForConditionalGeneration.from_pretrained(
    "llava-hf/LLaVA-NeXT-Video-7B-hf",
    quantization_config=quantization_config,
    device_map='auto'
)

import av
import numpy as np

def read_video_pyav(container, indices):
    '''
    Decode the video with PyAV decoder.

    Args:
        container (av.container.input.InputContainer): PyAV container.
        indices (List[int]): List of frame indices to decode.

    Returns:
        np.ndarray: np array of decoded frames of shape (num_frames, height, width, 3).
    '''
    frames = []
    container.seek(0)
    start_index = indices[0]
    end_index = indices[-1]
    for i, frame in enumerate(container.decode(video=0)):
        if i > end_index:
            break
        if i >= start_index and i in indices:
            frames.append(frame)
    return np.stack([x.to_ndarray(format="rgb24") for x in frames])

from huggingface_hub import hf_hub_download

# Download video from the hub
video_path_1 = hf_hub_download(repo_id="raushan-testing-hf/videos-test", filename="sample_demo_1.mp4", repo_type="dataset")
video_path_2 = hf_hub_download(repo_id="raushan-testing-hf/videos-test", filename="karate.mp4", repo_type="dataset")

container = av.open(video_path_1)

# sample uniformly 8 frames from the video (we can sample more for longer videos)
total_frames = container.streams.video[0].frames
indices = np.arange(0, total_frames, total_frames / 8).astype(int)
clip_baby = read_video_pyav(container, indices)


container = av.open(video_path_2)

# sample uniformly 8 frames from the video (we can sample more for longer videos)
total_frames = container.streams.video[0].frames
indices = np.arange(0, total_frames, total_frames / 8).astype(int)
clip_karate = read_video_pyav(container, indices)


import requests
from PIL import Image

image_stop = Image.open(requests.get("https://www.ilankelman.org/stopsigns/australia.jpg", stream=True).raw)
image_snowman = Image.open(requests.get("https://huggingface.co/microsoft/kosmos-2-patch14-224/resolve/main/snowman.jpg", stream=True).raw)

import requests

image_stop = Image.open(requests.get("https://www.ilankelman.org/stopsigns/australia.jpg", stream=True).raw)
image_snowman = Image.open(requests.get("https://huggingface.co/microsoft/kosmos-2-patch14-224/resolve/main/snowman.jpg", stream=True).raw)

# Each "content" is a list of dicts and you can add image/video/text modalities
conversation = [
      {
          "role": "user",
          "content": [
              {"type": "text", "text": "Why is this video funny?"},
              {"type": "video"},
              ],
      },
]

conversation_2 = [
      {
          "role": "user",
          "content": [
              {"type": "text", "text": "What do you see in this video?"},
              {"type": "video"},
              ],
      },
]

prompt = processor.apply_chat_template(conversation, add_generation_prompt=True)
prompt_2 = processor.apply_chat_template(conversation_2, add_generation_prompt=True)

inputs = processor([prompt, prompt_2], videos=[clip_baby, clip_karate], padding=True, return_tensors="pt").to(model.device)

generate_kwargs = {"max_new_tokens": 100, "do_sample": True, "top_p": 0.9}

output = model.generate(**inputs, **generate_kwargs)
generated_text = processor.batch_decode(output, skip_special_tokens=True)

print(generated_text)