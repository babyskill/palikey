#pragma once

#ifndef PALIKEY_TSF_GUIDS_H
#define PALIKEY_TSF_GUIDS_H

#include <windows.h>
#include <initguid.h>

// Primary Text Service CLSID: {D67F188D-4418-44AA-A564-E007D2C0DD1C}
DEFINE_GUID(CLSID_PaliKeyTextService,
    0xd67f188d, 0x4418, 0x44aa, 0xa5, 0x64, 0xe0, 0x07, 0xd2, 0xc0, 0xdd, 0x1c);

// TSF Language Profile GUID: {5B556F97-994A-4D54-932C-89DE95337D5A}
DEFINE_GUID(GUID_PaliKeyProfile,
    0x5b556f97, 0x994a, 0x4d54, 0x93, 0x2c, 0x89, 0xde, 0x95, 0x33, 0x7d, 0x5a);

// Display Attribute GUID: {E4449DB3-F588-4C20-8B16-D97640392682}
DEFINE_GUID(GUID_PaliKeyDisplayAttribute,
    0xe4449db3, 0xf588, 0x4c20, 0x8b, 0x16, 0xd9, 0x76, 0x40, 0x39, 0x26, 0x82);

// Language Bar Item GUID: {847F8420-4349-42F3-A055-0D83B0B3E0C8}
DEFINE_GUID(GUID_PaliKeyLangBarItem,
    0x847f8420, 0x4349, 0x42f3, 0xa0, 0x55, 0x0d, 0x83, 0xb0, 0xb3, 0xe0, 0xc8);

#endif // PALIKEY_TSF_GUIDS_H
