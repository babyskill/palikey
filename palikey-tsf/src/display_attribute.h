#pragma once

#ifndef PALIKEY_TSF_DISPLAY_ATTRIBUTE_H
#define PALIKEY_TSF_DISPLAY_ATTRIBUTE_H

#include <windows.h>
#include <msctf.h>

class CPaliKeyDisplayAttributeInfo : public ITfDisplayAttributeInfo {
public:
    CPaliKeyDisplayAttributeInfo();
    virtual ~CPaliKeyDisplayAttributeInfo();

    // IUnknown
    STDMETHODIMP QueryInterface(REFIID riid, void **ppvObj) override;
    STDMETHODIMP_(ULONG) AddRef() override;
    STDMETHODIMP_(ULONG) Release() override;

    // ITfDisplayAttributeInfo
    STDMETHODIMP GetGUID(GUID *pguid) override;
    STDMETHODIMP GetDescription(BSTR *pbstrDesc) override;
    STDMETHODIMP GetAttributeInfo(TF_DISPLAYATTRIBUTE *pda) override;
    STDMETHODIMP SetAttributeInfo(const TF_DISPLAYATTRIBUTE *pda) override;
    STDMETHODIMP Reset(void) override;

private:
    LONG m_cRef;
};

class CPaliKeyEnumDisplayAttributeInfo : public IEnumTfDisplayAttributeInfo {
public:
    CPaliKeyEnumDisplayAttributeInfo();
    virtual ~CPaliKeyEnumDisplayAttributeInfo();

    // IUnknown
    STDMETHODIMP QueryInterface(REFIID riid, void **ppvObj) override;
    STDMETHODIMP_(ULONG) AddRef() override;
    STDMETHODIMP_(ULONG) Release() override;

    // IEnumTfDisplayAttributeInfo
    STDMETHODIMP Clone(IEnumTfDisplayAttributeInfo **ppEnum) override;
    STDMETHODIMP Next(ULONG ulCount, ITfDisplayAttributeInfo **rgInfo, ULONG *pcFetched) override;
    STDMETHODIMP Reset(void) override;
    STDMETHODIMP Skip(ULONG ulCount) override;

private:
    LONG m_cRef;
    ULONG m_iIndex;
};

#endif // PALIKEY_TSF_DISPLAY_ATTRIBUTE_H
