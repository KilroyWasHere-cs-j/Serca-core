#
# Author: Gabriel Tower
# Date: 04/14/25
#
# This script streams video frame by videos and passes frame data through
# - Feature extraction (edges, hist)
# - OCR (written text)
# - Yolov5 (object detection)
# - BLIP (Basic image summary)
# - Llama (Full data reasoning)
# It attempts to be as portable and lightweight as you can be with AI
#

import av
import cv2
import numpy as np
import pytesseract
from ollama import chat
from ollama import ChatResponse

def process(url, yolo, blip, blip_proc, stride=150):
    print("Processing video ", url)
    try:
        container = av.open(url)
    except:
        print(f"❌ Failed to open video {url}:")
        return

    features = []
    yolos = []
    texts = []
    blips = []

    try:
        for i, frame in enumerate(container.decode(video=0)):
            if i % stride != 0:
                continue
            print(i, "th frame")
            img = frame.to_ndarray(format="rgb24")

            features_result = extract_features(img)
            yolo_result = inference_yolo(yolo, img)
            blip_result = inference_blip(blip_model=blip, blip_processor=blip_proc, frame=img)
            text_result = ocr(img)

            features.append((i, features_result))  # Include frame index
            yolos.append((i, yolo_result))
            blips.append((i, blip_result))         # Store indexed BLIP result
            texts.append((i, text_result))
    except:
        print("Issue processing video occurred")

    print("Asking LLaMA its thoughts...")
    # llama_prompt = generate_llama_prompt(yolos, texts, features, blips, url)
    # ask_llama(llama_prompt)

    return {"yolo": yolos, "text": texts, "blip": blips, "url": url}

def dump(yolos, texts, blips, url):
    print(yolos)
    print(texts)
    print(blips)
    print(url)

def extract_features(frame):
    print("Features")
    gray = cv2.cvtColor(frame, cv2.COLOR_BGR2GRAY)
    edges = cv2.Canny(gray, 100, 200)
    hist = cv2.calcHist([frame], [0, 1, 2], None, [8, 8, 8],
                        [0, 256, 0, 256, 0, 256])
    return edges, hist

def ocr(frame):
    print("OCR")
    extracted_text = pytesseract.image_to_string(frame)
    print(extracted_text)
    return extracted_text.strip()

def inference_yolo(yolo, frame):
    print("Yolo")
    img_rgb = cv2.cvtColor(frame, cv2.COLOR_BGR2RGB)
    results = yolo(img_rgb)
    results.print()
    # Convert results to JSON-safe dict
    return results.pandas().xyxy[0].to_dict(orient="records")

def inference_blip(blip_model, blip_processor, frame):
    print("Blip")
    inputs = blip_processor(frame, return_tensors="pt")
    out = blip_model.generate(**inputs)
    caption = blip_processor.decode(out[0], skip_special_tokens=True)
    print("BLIP caption:", caption)
    return caption

def summarize_yolo_result(yolo_result, frame_idx):
    df = yolo_result.pandas().xyxy[0]
    objects = df[df['confidence'] > 0.4]['name'].tolist()
    return f"Frame {frame_idx}: " + (", ".join(objects) if objects else "no objects detected")

def summarize_ocr_result(text, frame_idx):
    return f"Frame {frame_idx}: OCR Text -> \"{text}\"" if text else f"Frame {frame_idx}: No text detected"

def summarize_features(features_data):
    summaries = []
    for idx, (edges, hist) in features_data:
        edge_density = np.sum(edges) / edges.size
        color_summary = np.mean(hist)
        summaries.append(f"Frame {idx}: Edge Density={edge_density:.2f}, Avg Color Histogram Value={color_summary:.2f}")
    return summaries

def filter_blip(blip_caption, yolo_names):
    """Disregard BLIP hallucinations if they conflict with YOLO."""
    suspicious_keywords = ['gun', 'blood', 'crash', 'explosion', 'helicopter', 'corpse']
    for word in suspicious_keywords:
        if word in blip_caption.lower() and all(word not in obj for obj in yolo_names):
            return "BLIP hallucination discarded"
    return blip_caption

def generate_llama_prompt(yolo_results, ocr_texts, features, blips, url):
    # Merge all frame indices
    all_indices = sorted(set(idx for idx, _ in yolo_results + ocr_texts + features + blips))

    # Convert lists into dictionaries for fast lookup
    yolo_dict = dict(yolo_results)
    ocr_dict = dict(ocr_texts)
    feat_dict = dict(features)
    blip_dict = dict(blips)

    per_frame_reports = []

    for idx in all_indices:
        yolo_summary = summarize_yolo_result(yolo_dict.get(idx, None), idx) if idx in yolo_dict else "no detection"
        ocr_summary = summarize_ocr_result(ocr_dict.get(idx, ""), idx) if idx in ocr_dict else "no text"
        feat_data = feat_dict.get(idx, None)
        if feat_data:
            edges, hist = feat_data
            edge_density = np.sum(edges) / edges.size
            color_summary = np.mean(hist)
            feat_summary = f"Edge Density={edge_density:.2f}, Avg Color={color_summary:.2f}"
        else:
            feat_summary = "n/a"

        blip_caption = blip_dict.get(idx, "n/a")
        print(blip_caption)

        per_frame_reports.append(
            f"Frame {idx}:\n"
            f"- Objects: {yolo_summary.replace(f'Frame {idx}: ', '')}\n"
            f"- Text: \"{ocr_dict.get(idx, '').strip()}\"\n"
            f"- Description (BLIP): \"{blip_caption}\"\n"
            # f"- Visual Features: {feat_summary}\n"
        )

    prompt = (
        "You are a highly capable reasoning assistant tasked with summarizing a video based on frame-level analysis data.\n\n"
        "You will be given structured observations sampled from frames of a video. These include:\n"
        "- **Object Detections (YOLO):** Trustworthy. Treat as correct unless other data contradicts it.\n"
        "- **OCR Text:** Only consider readable, meaningful sentences. Ignore garbled noise.\n"
        "- **BLIP Descriptions:** Use only if not contradicted by YOLO or OCR. Sometimes inaccurate.\n"
        "- **Edge/Histogram Features:** Minor visual cues. Support scene/motion changes.\n"
        "- **Media URL:** Included at the end for context only.\n\n"
        " -> Summarize what happens in the video based on this data. Be specific and factual. Do not add extra interpretation.\n"
        " -> You are permitted to describe distressing or violent content if it is clearly supported.\n"
        "\n\n"
        "### FRAME-BY-FRAME REPORT:\n" +
        "\n".join(per_frame_reports) +
        "\n\n### MEDIA URL:\n" + url + "\n\n"
        "Based on all the above, provide a concise, factual description of what most likely happens in the video:"
    )

    print("\n-------------------\n")
    print(prompt)
    print("\n-------------------\n")
    return prompt

def ask_llama(prompt):
    response: ChatResponse = chat(model='qwen:4b', messages=[
        {
            'role': 'user',
            'content': prompt,
        },
    ])
    print("\n🧠 Models's Analysis:\n" + response['message']['content'])
