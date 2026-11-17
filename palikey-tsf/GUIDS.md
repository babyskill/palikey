# PaliKey Windows TSF — GUID Registry & Documentation

This document records all globally unique identifiers (GUIDs / CLSIDs) used by the **PaliKey Text Services Framework (TSF)** text service COM in-process server (`PaliKeyTSF.dll`).

All GUIDs are RFC 4122 version 4 UUIDs generated specifically for PaliKey.

---

## 1. Primary Text Service CLSID (`CLSID_PaliKeyTextService`)

- **String Representation**: `{D67F188D-4418-44AA-A564-E007D2C0DD1C}`
- **Registry Key**: `HKEY_CLASSES_ROOT\CLSID\{D67F188D-4418-44AA-A564-E007D2C0DD1C}`
- **Display Name**: `"PaliKey Pāḷi Text Service"`
- **C/C++ Definition**:
  ```cpp
  // {D67F188D-4418-44AA-A564-E007D2C0DD1C}
  DEFINE_GUID(CLSID_PaliKeyTextService,
      0xd67f188d, 0x4418, 0x44aa, 0xa5, 0x64, 0xe0, 0x07, 0xd2, 0xc0, 0xdd, 0x1c);
  ```
- **Purpose**:
  - Registered as the COM InprocServer32 CLSID.
  - Implements `ITfTextInputProcessor` and `ITfTextInputProcessorEx`.
  - Registered in TSF category `GUID_TFCAT_TIP_KEYBOARD`.

---

## 2. Language Profile GUID (`GUID_PaliKeyProfile`)

- **String Representation**: `{5B556F97-994A-4D54-932C-89DE95337D5A}`
- **Display Name**: `"PaliKey (Pāḷi)"`
- **C/C++ Definition**:
  ```cpp
  // {5B556F97-994A-4D54-932C-89DE95337D5A}
  DEFINE_GUID(GUID_PaliKeyProfile,
      0x5b556f97, 0x994a, 0x4d54, 0x93, 0x2c, 0x89, 0xde, 0x95, 0x33, 0x7d, 0x5a);
  ```
- **Purpose**:
  - Registered via `ITfInputProcessorProfiles::AddLanguageProfile`.
  - Associated with base language ID `0x0409` (`MAKELANGID(LANG_ENGLISH, SUBLANG_ENGLISH_US)`).
  - Identifies PaliKey in the Windows Language Bar / Input Indicator menu (Win + Space).

---

## 3. Display Attribute GUID (`GUID_PaliKeyDisplayAttribute`)

- **String Representation**: `{E4449DB3-F588-4C20-8B16-D97640392682}`
- **Description**: `"PaliKey Preedit Display Attribute"`
- **C/C++ Definition**:
  ```cpp
  // {E4449DB3-F588-4C20-8B16-D97640392682}
  DEFINE_GUID(GUID_PaliKeyDisplayAttribute,
      0xe4449db3, 0xf588, 0x4c20, 0x8b, 0x16, 0xd9, 0x76, 0x40, 0x39, 0x26, 0x82);
  ```
- **Purpose**:
  - Registered via `ITfCategoryMgr::RegisterCategory` with `GUID_TFCAT_DISPLAYATTRIBUTEPROVIDER`.
  - Exposed via `ITfDisplayAttributeProvider::GetDisplayAttributeInfo`.
  - Defines the visual feedback (dotted underline `TF_LS_DOT` with `TF_ATTR_INPUT`) rendered under active preedit/composition ranges in target applications (Notepad, Word, Edge, Chrome).

---

## 4. Language Bar / Compartment GUID (`GUID_PaliKeyLangBarItem`)

- **String Representation**: `{847F8420-4349-42F3-A055-0D83B0B3E0C8}`
- **Description**: `"PaliKey Language Bar Item"`
- **C/C++ Definition**:
  ```cpp
  // {847F8420-4349-42F3-A055-0D83B0B3E0C8}
  DEFINE_GUID(GUID_PaliKeyLangBarItem,
      0x847f8420, 0x4349, 0x42f3, 0xa0, 0x55, 0x0d, 0x83, 0xb0, 0xb3, 0xe0, 0xc8);
  ```
- **Purpose**:
  - Identifies PaliKey's button and status compartment on the Windows taskbar and language bar.

---

## 5. Standard Windows TSF Categories Registered

During `DllRegisterServer()`, PaliKey registers `CLSID_PaliKeyTextService` under the following system category GUIDs:

| Category GUID | Name | Purpose |
|---------------|------|---------|
| `{C3AC0B26-318B-4264-A42E-771A0E7F1CD9}` | `GUID_TFCAT_TIP_KEYBOARD` | Declares PaliKey as a keyboard text input processor. |
| `{046B8C25-9957-4FB0-B5F0-96F32A3D56AB}` | `GUID_TFCAT_DISPLAYATTRIBUTEPROVIDER` | Declares PaliKey as a provider of composition underline styles. |
| `{49D211DD-0E76-47A4-9E04-A78519E63E3D}` | `GUID_TFCAT_TIPCAP_UIELEMENTENABLED` | Declares support for custom and modern UI element display. |
| `{7ED423CB-982A-4A42-BB15-AB9E42BE33E4}` | `GUID_TFCAT_TIPCAP_SECUREMODE` | Enables operation in secure desktops (UAC elevation prompts, lock screen). |
| `{13A016DF-560B-46CD-947A-4C370F77037D}` | `GUID_TFCAT_TIPCAP_IMMERSIVESUPPORT` | Enables support in Windows 8/10/11 UWP, WinRT, and packaged Windows apps. |
| `{50D4509F-7250-45CB-AA00-07110A8E2D33}` | `GUID_TFCAT_TIPCAP_SYSTRAYSUPPORT` | Enables integration with the modern Windows taskbar system tray input switch. |
