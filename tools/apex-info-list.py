#!/usr/bin/env python3
"""Writes <rom>/apex/apex-info-list.xml (normally produced by apexd) from the
activated APEX directories created by fetch-rom.sh."""
import os, sys

def varint(b, i):
    v = s = 0
    while True:
        c = b[i]; i += 1
        v |= (c & 0x7f) << s; s += 7
        if c < 0x80:
            return v, i

def manifest(path):
    b = open(path, 'rb').read(); i = 0; out = {}
    while i < len(b):
        key, i = varint(b, i); f, t = key >> 3, key & 7
        if t == 0:
            v, i = varint(b, i)
        elif t == 2:
            n, i = varint(b, i); v = b[i:i + n]; i += n
        elif t == 5:
            v = b[i:i + 4]; i += 4
        elif t == 1:
            v = b[i:i + 8]; i += 8
        else:
            break
        out.setdefault(f, v)
    return out

rom = sys.argv[1]
apex = os.path.join(rom, 'apex')
src = {}
for d in ('system/apex', 'system_ext/apex', 'vendor/apex', 'product/apex'):
    p = os.path.join(rom, d)
    if os.path.isdir(p):
        for f in os.listdir(p):
            src[f] = '/' + d + '/' + f
lines = ['<?xml version="1.0" encoding="utf-8"?>', '<apex-info-list>']
for name in sorted(os.listdir(apex)):
    mp = os.path.join(apex, name, 'apex_manifest.pb')
    if not os.path.isfile(mp):
        continue
    m = manifest(mp)
    version = m.get(2, 1)
    path = next((v for k, v in src.items() if k.startswith(name + '.') or k.startswith(name.replace('com.android.', 'com.google.android.') + '.')), '/system/apex/%s.apex' % name)
    lines.append('    <apex-info moduleName="%s" modulePath="%s" preinstalledModulePath="%s" versionCode="%d" versionName="" isFactory="true" isActive="true" lastUpdateMillis="0" provideSharedApexLibs="false" />' % (name, path, path, version))
lines.append('</apex-info-list>')
open(os.path.join(apex, 'apex-info-list.xml'), 'w').write('\n'.join(lines) + '\n')
print('wrote', len(lines) - 3, 'apexes')
