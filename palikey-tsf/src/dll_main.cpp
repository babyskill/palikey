#include <windows.h>
#include <msctf.h>
#include "guids.h"
#include "class_factory.h"
#include "register.h"

HINSTANCE g_hInst = nullptr;
LONG g_cServerLocks = 0;

BOOL WINAPI DllMain(HINSTANCE hInstance, DWORD dwReason, LPVOID lpReserved) {
    (void)lpReserved;
    switch (dwReason) {
    case DLL_PROCESS_ATTACH:
        g_hInst = hInstance;
        DisableThreadLibraryCalls(hInstance);
        break;
    case DLL_PROCESS_DETACH:
        break;
    }
    return TRUE;
}

STDAPI DllGetClassObject(REFCLSID rclsid, REFIID riid, void **ppv) {
    if (!ppv) return E_INVALIDARG;
    *ppv = nullptr;

    if (IsEqualCLSID(rclsid, CLSID_PaliKeyTextService)) {
        CPaliKeyClassFactory *pFactory = new CPaliKeyClassFactory();
        if (!pFactory) {
            return E_OUTOFMEMORY;
        }
        HRESULT hr = pFactory->QueryInterface(riid, ppv);
        pFactory->Release();
        return hr;
    }

    return CLASS_E_CLASSNOTAVAILABLE;
}

STDAPI DllCanUnloadNow(void) {
    return (g_cServerLocks == 0) ? S_OK : S_FALSE;
}

STDAPI DllRegisterServer(void) {
    return RegisterServer();
}

STDAPI DllUnregisterServer(void) {
    return UnregisterServer();
}
