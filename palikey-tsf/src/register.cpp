#include "register.h"
#include "guids.h"
#include <msctf.h>
#include <shlwapi.h>
#include <cstdio>

extern HINSTANCE g_hInst;

static const WCHAR c_szClsidKeyPrefix[] = L"CLSID\\{D67F188D-4418-44AA-A564-E007D2C0DD1C}";
static const WCHAR c_szInprocServer32[] = L"InprocServer32";
static const WCHAR c_szDescription[] = L"PaliKey Pāḷi Text Service";
static const WCHAR c_szThreadingModel[] = L"Apartment";

static const struct {
    const GUID *pCatGuid;
} c_Categories[] = {
    { &GUID_TFCAT_TIP_KEYBOARD },
    { &GUID_TFCAT_DISPLAYATTRIBUTEPROVIDER },
    { &GUID_TFCAT_TIPCAP_UIELEMENTENABLED },
    { &GUID_TFCAT_TIPCAP_SECUREMODE },
    { &GUID_TFCAT_TIPCAP_IMMERSIVESUPPORT },
    { &GUID_TFCAT_TIPCAP_SYSTRAYSUPPORT }
};

HRESULT RegisterServer() {
    WCHAR szDllPath[MAX_PATH] = {0};
    if (GetModuleFileNameW(g_hInst, szDllPath, MAX_PATH) == 0) {
        return HRESULT_FROM_WIN32(GetLastError());
    }

    // 1. Register COM InprocServer32 in HKEY_CLASSES_ROOT
    HKEY hKey = nullptr;
    LONG lRes = RegCreateKeyExW(HKEY_CLASSES_ROOT, c_szClsidKeyPrefix, 0, nullptr,
                                REG_OPTION_NON_VOLATILE, KEY_WRITE, nullptr, &hKey, nullptr);
    if (lRes != ERROR_SUCCESS) {
        return HRESULT_FROM_WIN32(lRes);
    }
    RegSetValueExW(hKey, nullptr, 0, REG_SZ, (const BYTE*)c_szDescription,
                   (DWORD)(wcslen(c_szDescription) + 1) * sizeof(WCHAR));

    HKEY hSubKey = nullptr;
    lRes = RegCreateKeyExW(hKey, c_szInprocServer32, 0, nullptr,
                           REG_OPTION_NON_VOLATILE, KEY_WRITE, nullptr, &hSubKey, nullptr);
    if (lRes == ERROR_SUCCESS) {
        RegSetValueExW(hSubKey, nullptr, 0, REG_SZ, (const BYTE*)szDllPath,
                       (DWORD)(wcslen(szDllPath) + 1) * sizeof(WCHAR));
        RegSetValueExW(hSubKey, L"ThreadingModel", 0, REG_SZ, (const BYTE*)c_szThreadingModel,
                       (DWORD)(wcslen(c_szThreadingModel) + 1) * sizeof(WCHAR));
        RegCloseKey(hSubKey);
    }
    RegCloseKey(hKey);

    // 2. Register with TSF InputProcessorProfiles
    ITfInputProcessorProfiles *pProfiles = nullptr;
    HRESULT hr = CoCreateInstance(CLSID_TF_InputProcessorProfiles, nullptr,
                                  CLSCTX_INPROC_SERVER, IID_ITfInputProcessorProfiles,
                                  (void**)&pProfiles);
    if (SUCCEEDED(hr)) {
        hr = pProfiles->Register(CLSID_PaliKeyTextService);
        if (SUCCEEDED(hr)) {
            hr = pProfiles->AddLanguageProfile(
                CLSID_PaliKeyTextService,
                MAKELANGID(LANG_ENGLISH, SUBLANG_ENGLISH_US),
                GUID_PaliKeyProfile,
                c_szDescription,
                (ULONG)wcslen(c_szDescription),
                szDllPath,
                (ULONG)wcslen(szDllPath),
                0
            );
        }
        pProfiles->Release();
    }

    // 3. Register with TSF Category Manager
    ITfCategoryMgr *pCatMgr = nullptr;
    hr = CoCreateInstance(CLSID_TF_CategoryMgr, nullptr,
                          CLSCTX_INPROC_SERVER, IID_ITfCategoryMgr,
                          (void**)&pCatMgr);
    if (SUCCEEDED(hr)) {
        for (size_t i = 0; i < sizeof(c_Categories) / sizeof(c_Categories[0]); ++i) {
            pCatMgr->RegisterCategory(CLSID_PaliKeyTextService,
                                      *c_Categories[i].pCatGuid,
                                      CLSID_PaliKeyTextService);
        }
        pCatMgr->Release();
    }

    return S_OK;
}

HRESULT UnregisterServer() {
    // 1. Unregister Categories
    ITfCategoryMgr *pCatMgr = nullptr;
    if (SUCCEEDED(CoCreateInstance(CLSID_TF_CategoryMgr, nullptr,
                                  CLSCTX_INPROC_SERVER, IID_ITfCategoryMgr,
                                  (void**)&pCatMgr))) {
        for (size_t i = 0; i < sizeof(c_Categories) / sizeof(c_Categories[0]); ++i) {
            pCatMgr->UnregisterCategory(CLSID_PaliKeyTextService,
                                        *c_Categories[i].pCatGuid,
                                        CLSID_PaliKeyTextService);
        }
        pCatMgr->Release();
    }

    // 2. Unregister Profile
    ITfInputProcessorProfiles *pProfiles = nullptr;
    if (SUCCEEDED(CoCreateInstance(CLSID_TF_InputProcessorProfiles, nullptr,
                                  CLSCTX_INPROC_SERVER, IID_ITfInputProcessorProfiles,
                                  (void**)&pProfiles))) {
        pProfiles->RemoveLanguageProfile(CLSID_PaliKeyTextService,
                                         MAKELANGID(LANG_ENGLISH, SUBLANG_ENGLISH_US),
                                         GUID_PaliKeyProfile);
        pProfiles->Unregister(CLSID_PaliKeyTextService);
        pProfiles->Release();
    }

    // 3. Remove COM Registry keys
    RegDeleteTreeW(HKEY_CLASSES_ROOT, c_szClsidKeyPrefix);

    return S_OK;
}
