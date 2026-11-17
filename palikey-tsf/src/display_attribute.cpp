#include "display_attribute.h"
#include "guids.h"

// ----------------------------------------------------------------------------
// CPaliKeyDisplayAttributeInfo
// ----------------------------------------------------------------------------

CPaliKeyDisplayAttributeInfo::CPaliKeyDisplayAttributeInfo()
    : m_cRef(1) {
}

CPaliKeyDisplayAttributeInfo::~CPaliKeyDisplayAttributeInfo() {
}

STDMETHODIMP CPaliKeyDisplayAttributeInfo::QueryInterface(REFIID riid, void **ppvObj) {
    if (!ppvObj) return E_INVALIDARG;
    *ppvObj = nullptr;

    if (IsEqualIID(riid, IID_IUnknown) || IsEqualIID(riid, IID_ITfDisplayAttributeInfo)) {
        *ppvObj = static_cast<ITfDisplayAttributeInfo*>(this);
        AddRef();
        return S_OK;
    }
    return E_NOINTERFACE;
}

STDMETHODIMP_(ULONG) CPaliKeyDisplayAttributeInfo::AddRef() {
    return InterlockedIncrement(&m_cRef);
}

STDMETHODIMP_(ULONG) CPaliKeyDisplayAttributeInfo::Release() {
    ULONG count = InterlockedDecrement(&m_cRef);
    if (count == 0) {
        delete this;
    }
    return count;
}

STDMETHODIMP CPaliKeyDisplayAttributeInfo::GetGUID(GUID *pguid) {
    if (!pguid) return E_INVALIDARG;
    *pguid = GUID_PaliKeyDisplayAttribute;
    return S_OK;
}

STDMETHODIMP CPaliKeyDisplayAttributeInfo::GetDescription(BSTR *pbstrDesc) {
    if (!pbstrDesc) return E_INVALIDARG;
    *pbstrDesc = SysAllocString(L"PaliKey Composition Attribute");
    return (*pbstrDesc) ? S_OK : E_OUTOFMEMORY;
}

STDMETHODIMP CPaliKeyDisplayAttributeInfo::GetAttributeInfo(TF_DISPLAYATTRIBUTE *pda) {
    if (!pda) return E_INVALIDARG;

    // Dotted underline for in-progress preedit composition
    pda->crText.type = TF_CT_NONE;
    pda->crBk.type = TF_CT_NONE;
    pda->lsStyle = TF_LS_DOT;
    pda->fBoldLine = FALSE;
    pda->crLine.type = TF_CT_SYSCOLOR;
    pda->crLine.nIndex = COLOR_WINDOWTEXT;
    pda->bAttr = TF_ATTR_INPUT;

    return S_OK;
}

STDMETHODIMP CPaliKeyDisplayAttributeInfo::SetAttributeInfo(const TF_DISPLAYATTRIBUTE *pda) {
    (void)pda;
    return E_NOTIMPL;
}

STDMETHODIMP CPaliKeyDisplayAttributeInfo::Reset(void) {
    return S_OK;
}

// ----------------------------------------------------------------------------
// CPaliKeyEnumDisplayAttributeInfo
// ----------------------------------------------------------------------------

CPaliKeyEnumDisplayAttributeInfo::CPaliKeyEnumDisplayAttributeInfo()
    : m_cRef(1), m_iIndex(0) {
}

CPaliKeyEnumDisplayAttributeInfo::~CPaliKeyEnumDisplayAttributeInfo() {
}

STDMETHODIMP CPaliKeyEnumDisplayAttributeInfo::QueryInterface(REFIID riid, void **ppvObj) {
    if (!ppvObj) return E_INVALIDARG;
    *ppvObj = nullptr;

    if (IsEqualIID(riid, IID_IUnknown) || IsEqualIID(riid, IID_IEnumTfDisplayAttributeInfo)) {
        *ppvObj = static_cast<IEnumTfDisplayAttributeInfo*>(this);
        AddRef();
        return S_OK;
    }
    return E_NOINTERFACE;
}

STDMETHODIMP_(ULONG) CPaliKeyEnumDisplayAttributeInfo::AddRef() {
    return InterlockedIncrement(&m_cRef);
}

STDMETHODIMP_(ULONG) CPaliKeyEnumDisplayAttributeInfo::Release() {
    ULONG count = InterlockedDecrement(&m_cRef);
    if (count == 0) {
        delete this;
    }
    return count;
}

STDMETHODIMP CPaliKeyEnumDisplayAttributeInfo::Clone(IEnumTfDisplayAttributeInfo **ppEnum) {
    if (!ppEnum) return E_INVALIDARG;
    *ppEnum = nullptr;

    CPaliKeyEnumDisplayAttributeInfo *pClone = new CPaliKeyEnumDisplayAttributeInfo();
    if (!pClone) return E_OUTOFMEMORY;
    pClone->m_iIndex = m_iIndex;
    *ppEnum = pClone;
    return S_OK;
}

STDMETHODIMP CPaliKeyEnumDisplayAttributeInfo::Next(ULONG ulCount, ITfDisplayAttributeInfo **rgInfo, ULONG *pcFetched) {
    if (pcFetched) *pcFetched = 0;
    if (ulCount == 0 || !rgInfo) return S_OK;

    ULONG cFetched = 0;
    while (cFetched < ulCount && m_iIndex < 1) {
        rgInfo[cFetched] = new CPaliKeyDisplayAttributeInfo();
        if (!rgInfo[cFetched]) return E_OUTOFMEMORY;
        m_iIndex++;
        cFetched++;
    }

    if (pcFetched) *pcFetched = cFetched;
    return (cFetched == ulCount) ? S_OK : S_FALSE;
}

STDMETHODIMP CPaliKeyEnumDisplayAttributeInfo::Reset(void) {
    m_iIndex = 0;
    return S_OK;
}

STDMETHODIMP CPaliKeyEnumDisplayAttributeInfo::Skip(ULONG ulCount) {
    m_iIndex += ulCount;
    if (m_iIndex > 1) {
        m_iIndex = 1;
        return S_FALSE;
    }
    return S_OK;
}
