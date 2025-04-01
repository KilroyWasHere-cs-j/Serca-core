import torch

def validate():
    print(torch.cuda.is_available())
    print(torch.cuda.get_device_name(0))
    print(torch.__version__)
    print(torch.version.cuda)
    print(torch.backends.cudnn.version())
    print(f"Available VRAM: {torch.cuda.mem_get_info(0)[0] / 1e9:.2f} GB")