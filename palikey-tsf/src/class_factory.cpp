#include "class_factory.h"
#include "palikey_tsf.h"

extern LONG g_cServerLocks;

CPaliKeyClassFactory::CPaliKeyClassFactory()
    : m_cRef(1) {
}

CPaliKeyClassFactory::~CPaliKeyClassFactory() {
}

STDMETHODIMP CPaliKeyClassFactory::QueryInterface(REFIID riid, void **ppvObj) {
    if (!ppvObj) return E_INVALIDARG;
    *ppvObj = nullptr;

    if (IsEqualIID(riid, IID_IUnknown) || IsEqualIID(riid, IID_IClassFactory)) {
        *ppvObj = static_cast<IClassFactory*>(this);
        AddRef();
        return S_OK;
    }

    return E_NOINTERFACE;
}

STDMETHODIMP_(ULONG) CPaliKeyClassFactory::AddRef() {
    return InterlockedIncrement(&m_cRef);
}

STDMETHODIMP_(ULONG) CPaliKeyClassFactory::Release() {
    ULONG count = InterlockedDecrement(&m_cRef);
    if (count == 0) {
        delete this;
    }
    return count;
}

STDMETHODIMP CPaliKeyClassFactory::CreateInstance(IUnknown *pUnkOuter, REFIID riid, void **ppvObject) {
    if (!ppvObject) return E_INVALIDARG;
    *ppvObject = nullptr;

    if (pUnkOuter != nullptr) {
        return CLASS_E_NOAGGREGATION;
    }

    CPaliKeyTextService *pTextService = new CPaliKeyTextService();
    if (!pTextService) {
        return E_OUTOFMEMORY;
    }

    HRESULT hr = pTextService->QueryInterface(riid, ppvObject);
    pTextService->Release();
    return hr;
}

STDMETHODIMP CPaliKeyClassFactory::LockServer(BOOL fLock) {
    if (fLock) {
        InterlockedIncrement(&g_cServerLocks);
    } else {
        InterlockedDecrement(&g_cServerLocks);
    }
    return S_OK;
}
