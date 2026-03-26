const fs = require('fs');
let content = fs.readFileSync('src/hooks/useItemReorder.ts', 'utf-8');

if (!content.includes('import { useCallback, useEffect, useRef, useState } from "react";')) {
    content = content.replace(
        /import \{ useCallback, useEffect, useRef, useState \} from "react";/,
        'import { useCallback, useEffect, useRef, useState } from "react";'
    );
}

fs.writeFileSync('src/hooks/useItemReorder.ts', content);
