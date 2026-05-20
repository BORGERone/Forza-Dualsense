<p align="center">
<img width="256" height="256" alt="icon" src="https://github.com/user-attachments/assets/408a0795-5182-40a2-b939-631545007050" />
</p>

# Forza DualSense
<p align="center">
  <img width="1202" height="802" alt="Screenshot" src="https://github.com/user-attachments/assets/4968f25e-8b51-499b-b968-292f5c191fb9" />

  <i>Forza Horizon में DualSense कंट्रोलर को प्रबंधित करने के लिए आधुनिक इंटरफेस</i>
</p>

---

## विवरण

Forza DualSense Forza Horizon में PlayStation 5 DualSense कंट्रोलर को प्रबंधित करने के लिए एक एप्लिकेशन है। यह प्रोग्राम एडेप्टिव ट्रिगर, हैप्टिक फीडबैक और अन्य हैप्टिक फीडबैक सुविधाओं सहित विस्तारित कंट्रोलर अनुकूलन क्षमताएं प्रदान करता है।

---

## निर्भरताएं

| निर्भरता | विवरण |
|-----------|--------|
| **[Python 3.12](https://apps.microsoft.com/detail/9ncvdn91xzqp)** | बैकएंड स्क्रिप्ट के लिए आवश्यक |

---

## स्थापना

### आवश्यकताएं
- **[Python 3.12](https://apps.microsoft.com/detail/9ncvdn91xzqp)** या नया
- DualSense कंट्रोलर (PS5)

### लॉन्च
1. DualSense कंट्रोलर को अपने कंप्यूटर से कनेक्ट करें
2. `forza-dualsense.exe` चलाएं
3. एप्लिकेशन इंटरफेस में पैरामीटर कॉन्फ़िगर करें
4. गेम .exe पथ निर्दिष्ट करें (वैकल्पिक, स्वचालित लॉन्च के लिए आवश्यक)

## गेम में सेटअप

Forza Horizon खोलें → **सेटिंग्स → HUD और गेमप्ले**, नीचे स्क्रॉल करें:

| सेटिंग | मान |
|---------|-----|
| Data Out | **ON** |
| Data Out IP Address | **127.0.0.1** |
| Data Out IP Port | **5300** |

---

## सुविधाएं

### कंट्रोलर नियंत्रण
- गैस और ब्रेक पेडल के लिए एडेप्टिव ट्रिगर कॉन्फ़िगरेशन
- हैप्टिक फीडबैक कॉन्फ़िगरेशन
- जॉयस्टिक के लिए डेडजोन सेटिंग्स
- ABS (एंटी-लॉक ब्रेकिंग सिस्टम) समर्थन
- कनेक्शन पुष्टि के लिए स्टार्टअप पल्स

### इंटरफेस
- सहज नियंत्रण के साथ आधुनिक डार्क डिज़ाइन
- कंट्रोलर कनेक्शन स्थिति का वास्तविक समय प्रदर्शन
- गेम संचार के लिए UDP पैरामीटर सेटिंग्स
- एप्लिकेशन से सीधे Forza Horizon लॉन्च करने की क्षमता (.exe पथ निर्दिष्ट होने पर)

---

## उपयोग

### प्रारंभिक सेटअप
1. एप्लिकेशन लॉन्च करें
2. सुनिश्चित करें कि कंट्रोलर कनेक्ट है
3. गेम संचार के लिए UDP पैरामीटर कॉन्फ़िगर करें
4. ट्रिगर और जॉयस्टिक संवेदनशीलता समायोजित करें
5. एप्लिकेशन बटन के माध्यम से Forza Horizon लॉन्च करें

---

## समस्या निवारण

### कंट्रोलर कनेक्ट नहीं हो रहा
- सुनिश्चित करें कि कंट्रोलर USB या ब्लूटूथ से कनेक्ट है
- तीसरे पक्ष के ड्राइवर (DS4W जैसे) का उपयोग करते समय, Steam Input को **<u>पूरी तरह</u>** अक्षम करें
- जांचें कि Python स्थापित है और काम कर रहा है
- एंटीवायरस अक्षम करें
- फ़ायरवॉल अक्षम करें
- डिफ़ॉल्ट पोर्ट (5300) की उपलब्धता की जांच करें, आवश्यक हो तो बदलें
- %LOCALAPPDATA%\Forza DualSense\backend फ़ोल्डर की जांच करें
- एप्लिकेशन पुनः प्रारंभ करें

---

## कॉन्फ़िगरेशन

बैकएंड फ़ाइल स्वचालित रूप से निम्नलिखित में निकाली जाती हैं:
```
%LOCALAPPDATA%\Forza DualSense\backend
```

एप्लिकेशन सेटिंग्स कॉन्फ़िगरेशन फ़ाइल में सहेजी जाती हैं।

---

## डेवलपर्स के लिए

### विकास निर्भरताएं

| निर्भरता | विवरण |
|-----------|--------|
| **[Node.js 18+](https://nodejs.org/)** | फ्रंटएंड विकास और बिल्ड के लिए आवश्यक |
| **[Rust 1.70+](https://www.rust-lang.org/tools/install)** | Tauri बैकएंड कंपाइलेशन के लिए आवश्यक |
| **[Python 3.12](https://www.python.org/downloads/)** | बैकएंड स्क्रिप्ट के लिए आवश्यक |

### निर्भरताएं स्थापित करें

#### Frontend (Tauri)
```powershell
cd tauri-app
npm install
```

#### Backend (Python)
```powershell
cd backend
pip install -r requirements.txt
# या uv का उपयोग करें (अनुशंसित)
uv sync
```

### विकास
```powershell
cd tauri-app
npm run tauri dev
```

### रिलीज़ बिल्ड
```powershell
cd tauri-app
npm run tauri build
```

बिल्ड के बाद, exe फ़ाइल यहां होगी:
```
src-tauri/target/release/forza-dualsense.exe
```

---

## प्रोजेक्ट आर्किटेक्चर

```
tauri-app/
├── src/                 # Tauri Frontend (TypeScript/HTML)
├── src-tauri/           # Tauri Rust Backend
│   ├── backend/         # Python IPC Backend (exe में एम्बेडेड)
│   │   ├── modules/     # Python मॉड्यूल
│   │   ├── ipc_server.py
│   │   └── pyproject.toml
│   ├── src/             # Rust सोर्स कोड
│   │   └── main.rs      # IPC तर्क के साथ मुख्य फ़ाइल
│   ├── build.rs         # बिल्ड स्क्रिप्ट
│   └── tauri.conf.json  # Tauri कॉन्फ़िगरेशन
├── package.json         # Node.js निर्भरताएं
└── build.bat            # Windows बिल्ड स्क्रिप्ट
```

### एप्लिकेशन आर्किटेक्चर
- **Tauri Frontend** - TypeScript/HTML GUI इंटरफेस
- **Tauri Rust Backend** - प्रक्रिया प्रबंधन और IPC संचार
- **Python IPC Backend** - stdin/stdout के माध्यम से कंट्रोलर कमांड प्रोसेसिंग
- **एम्बेडेड संसाधन** - rust-embed के माध्यम से exe में एम्बेडेड बैकएंड फ़ाइल

---

## IPC प्रोटोकॉल

Python बैकएंड stdin/stdout के माध्यम से JSON का उपयोग करके Rust बैकएंड के साथ संवाद करता है:

### अनुरोध (stdin में)
```json
{"type": "start"}
{"type": "stop"}
{"type": "get_status"}
{"type": "update_setting", "key": "udp_port", "value": 5300}
```

### प्रतिक्रियाएं (stdout से)
```json
{"status": "started", "message": "Backend started successfully"}
{"type": "status", "dualsense_connected": true, "running": true, "settings": {...}}
```

---

## धन्यवाद

यह प्रोजेक्ट [Forza Horizon DualSense Python](https://github.com/HamzaYslmn/Forza-Horizon-DualSense-Python) by HamzaYslmn का Python बैकएंड कोड उपयोग करता है, जो AGPL v3 के तहत लाइसेंस प्राप्त है। मूल प्रोजेक्ट Forza Horizon के साथ DualSense कंट्रोलर संचार के लिए मुख्य कार्यक्षमता प्रदान करता है।

---
