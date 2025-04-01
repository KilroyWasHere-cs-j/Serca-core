import torch
import winreg as wrg 

def validate():
    print(torch.cuda.is_available())
    print(torch.cuda.get_device_name(0))
    print(torch.__version__)
    print(torch.version.cuda)
    print(torch.backends.cudnn.version())
    print(f"Available VRAM: {torch.cuda.mem_get_info(0)[0] / 1e9:.2f} GB")


def load():
    # Store location of HKEY_CURRENT_USER 
    location = wrg.HKEY_CURRENT_USER 
  
    # Storing path in soft 
    soft = wrg.OpenKeyEx(location,r"SOFTWARE\\Serca\\") 
  
    # reading values in value_1 and value_2 
    value_1 = wrg.QueryValueEx(soft,"AI_Prompt") 
    value_2 = wrg.QueryValueEx(soft,"AI_Model") 
  
    # Closing folder 
    if soft: 
        wrg.CloseKey(soft) 

    return value_1, value_2

def set():
    # Store location of HKEY_CURRENT_USER 
    location = wrg.HKEY_CURRENT_USER 
  
    # Store path in soft 
    soft = wrg.OpenKeyEx(location, r"SOFTWARE\\") 
    key_1 = wrg.CreateKey(soft, "Folder") 
  
    # Creating values in Geeks 
    wrg.SetValueEx(key_1, "Key", 0, wrg.REG_SZ, 
               "Value") 
  
    if key_1: 
        wrg.CloseKey(key_1) 
    print("Keys set")