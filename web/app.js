/**
 * Super-Grandmaster Chess Web App (2850 Elo)
 * Pure vanilla JavaScript with Web Audio API & MultiPV Live Engine Integration.
 */

// Embedded SVG Piece Vectors
const PIECE_SVGS = {
    'P': `<svg viewBox="0 0 45 45"><path d="m 22.5,9 c -2.21,0 -4,1.79 -4,4 0,0.89 0.29,1.71 0.78,2.38 C 17.33,16.5 16,18.59 16,21 c 0,2.03 0.94,3.84 2.41,5.03 C 15.41,27.09 11,31.58 11,39.5 l 23,0 c 0,-7.92 -4.41,-12.41 -7.41,-13.47 C 28.06,24.84 29,23.03 29,21 29,18.59 27.67,16.5 25.72,15.38 26.21,14.71 26.5,13.89 26.5,13 c 0,-2.21 -1.79,-4 -4,-4 z" fill="#fff" stroke="#000" stroke-width="1.5" stroke-linecap="round"/></svg>`,
    'N': `<svg viewBox="0 0 45 45"><path d="m 22,10 c 10.5,1 16.5,8 16,29 L 15,39 C 15,30 9.65,27.76 9,19 8.35,10.24 14,8 14,8 c 0,0 2.5,4 4.5,4 2,0 3,-2 3,-2 z" fill="#fff" stroke="#000" stroke-width="1.5"/><circle cx="15" cy="14" r="1.5" fill="#000"/></svg>`,
    'B': `<svg viewBox="0 0 45 45"><g fill="#fff" stroke="#000" stroke-width="1.5" stroke-linecap="round"><path d="M 9,36 C 12.39,35.03 19.11,36.43 22.5,34 C 25.89,36.43 32.61,35.03 36,36 c -4,4 -23,4 -27,0 z"/><path d="M 15,32 C 17.5,34.5 27.5,34.5 30,32 C 30.5,30.5 30,30 30,30 C 30,27.5 27.5,26 22.5,26 C 17.5,26 15,27.5 15,30 C 15,30 14.5,30.5 15,32 z"/><path d="m 22.5,9 c -4,0 -7,3 -7,7 0,3 2,6 7,10 5,-4 7,-7 7,-10 0,-4 -3,-7 -7,-7 z"/><circle cx="22.5" cy="6" r="2"/></g></svg>`,
    'R': `<svg viewBox="0 0 45 45"><g fill="#fff" stroke="#000" stroke-width="1.5" stroke-linecap="round"><path d="M 9,39 L 36,39 L 36,36 L 9,36 z"/><path d="M 12,36 L 12,32 L 33,32 L 33,36 z"/><path d="M 11,14 L 11,9 L 15,9 L 15,11 L 20,11 L 20,9 L 25,9 L 25,11 L 30,11 L 30,9 L 34,9 L 34,14 z"/><path d="M 34,14 L 31,17 L 14,17 L 11,14 z"/><path d="M 31,17 L 31,29.5 L 14,29.5 L 14,17 z"/></g></svg>`,
    'Q': `<svg viewBox="0 0 45 45"><g fill="#fff" stroke="#000" stroke-width="1.5" stroke-linecap="round"><path d="M 9,26 C 17.5,24.5 30,24.5 36,26 L 38.5,13.5 L 31,25 L 22.5,10 L 14,25 L 6.5,13.5 z"/><path d="M 9,26 L 9,36 L 36,36 L 36,26 z"/><circle cx="6" cy="12" r="2"/><circle cx="14" cy="9" r="2"/><circle cx="22.5" cy="6" r="2"/><circle cx="31" cy="9" r="2"/><circle cx="39" cy="12" r="2"/></g></svg>`,
    'K': `<svg viewBox="0 0 45 45"><g fill="#fff" stroke="#000" stroke-width="1.5" stroke-linecap="round"><path d="M 22.5,11.63 L 22.5,6 M 20,8 L 25,8"/><path d="M 22.5,25 C 22.5,25 27,17.5 25.5,14.5 C 24,11.5 21,11.5 22.5,25 z"/><path d="M 11.5,37 C 17,40.5 28,40.5 33.5,37 L 33.5,30 C 33.5,30 36.5,24.5 35,20.5 C 33.5,16.5 29.5,16.5 28,19 C 26.5,21.5 26.5,24 22.5,24 C 18.5,24 18.5,21.5 17,19 C 15.5,16.5 11.5,16.5 10,20.5 C 8.5,24.5 11.5,30 11.5,30 z"/></g></svg>`,

    'p': `<svg viewBox="0 0 45 45"><path d="m 22.5,9 c -2.21,0 -4,1.79 -4,4 0,0.89 0.29,1.71 0.78,2.38 C 17.33,16.5 16,18.59 16,21 c 0,2.03 0.94,3.84 2.41,5.03 C 15.41,27.09 11,31.58 11,39.5 l 23,0 c 0,-7.92 -4.41,-12.41 -7.41,-13.47 C 28.06,24.84 29,23.03 29,21 29,18.59 27.67,16.5 25.72,15.38 26.21,14.71 26.5,13.89 26.5,13 c 0,-2.21 -1.79,-4 -4,-4 z" fill="#262626" stroke="#fff" stroke-width="1.2" stroke-linecap="round"/></svg>`,
    'n': `<svg viewBox="0 0 45 45"><path d="m 22,10 c 10.5,1 16.5,8 16,29 L 15,39 C 15,30 9.65,27.76 9,19 8.35,10.24 14,8 14,8 c 0,0 2.5,4 4.5,4 2,0 3,-2 3,-2 z" fill="#262626" stroke="#fff" stroke-width="1.2"/><circle cx="15" cy="14" r="1.5" fill="#fff"/></svg>`,
    'b': `<svg viewBox="0 0 45 45"><g fill="#262626" stroke="#fff" stroke-width="1.2" stroke-linecap="round"><path d="M 9,36 C 12.39,35.03 19.11,36.43 22.5,34 C 25.89,36.43 32.61,35.03 36,36 c -4,4 -23,4 -27,0 z"/><path d="M 15,32 C 17.5,34.5 27.5,34.5 30,32 C 30.5,30.5 30,30 30,30 C 30,27.5 27.5,26 22.5,26 C 17.5,26 15,27.5 15,30 C 15,30 14.5,30.5 15,32 z"/><path d="m 22.5,9 c -4,0 -7,3 -7,7 0,3 2,6 7,10 5,-4 7,-7 7,-10 0,-4 -3,-7 -7,-7 z"/><circle cx="22.5" cy="6" r="2"/></g></svg>`,
    'r': `<svg viewBox="0 0 45 45"><g fill="#262626" stroke="#fff" stroke-width="1.2" stroke-linecap="round"><path d="M 9,39 L 36,39 L 36,36 L 9,36 z"/><path d="M 12,36 L 12,32 L 33,32 L 33,36 z"/><path d="M 11,14 L 11,9 L 15,9 L 15,11 L 20,11 L 20,9 L 25,9 L 25,11 L 30,11 L 30,9 L 34,9 L 34,14 z"/><path d="M 34,14 L 31,17 L 14,17 L 11,14 z"/><path d="M 31,17 L 31,29.5 L 14,29.5 L 14,17 z"/></g></svg>`,
    'q': `<svg viewBox="0 0 45 45"><g fill="#262626" stroke="#fff" stroke-width="1.2" stroke-linecap="round"><path d="M 9,26 C 17.5,24.5 30,24.5 36,26 L 38.5,13.5 L 31,25 L 22.5,10 L 14,25 L 6.5,13.5 z"/><path d="M 9,26 L 9,36 L 36,36 L 36,26 z"/><circle cx="6" cy="12" r="2"/><circle cx="14" cy="9" r="2"/><circle cx="22.5" cy="6" r="2"/><circle cx="31" cy="9" r="2"/><circle cx="39" cy="12" r="2"/></g></svg>`,
    'k': `<svg viewBox="0 0 45 45"><g fill="#262626" stroke="#fff" stroke-width="1.2" stroke-linecap="round"><path d="M 22.5,11.63 L 22.5,6 M 20,8 L 25,8"/><path d="M 22.5,25 C 22.5,25 27,17.5 25.5,14.5 C 24,11.5 21,11.5 22.5,25 z"/><path d="M 11.5,37 C 17,40.5 28,40.5 33.5,37 L 33.5,30 C 33.5,30 36.5,24.5 35,20.5 C 33.5,16.5 29.5,16.5 28,19 C 26.5,21.5 26.5,24 22.5,24 C 18.5,24 18.5,21.5 17,19 C 15.5,16.5 11.5,16.5 10,20.5 C 8.5,24.5 11.5,30 11.5,30 z"/></g></svg>`
};

// Sound Synthesizer via Web Audio API
class ChessAudio {
    constructor() {
        this.ctx = null;
    }

    init() {
        if (!this.ctx) {
            this.ctx = new (window.AudioContext || window.webkitAudioContext)();
        }
        if (this.ctx.state === 'suspended') {
            this.ctx.resume();
        }
    }

    playMove() {
        this.init();
        const osc = this.ctx.createOscillator();
        const gain = this.ctx.createGain();
        osc.type = 'sine';
        osc.frequency.setValueAtTime(180, this.ctx.currentTime);
        osc.frequency.exponentialRampToValueAtTime(40, this.ctx.currentTime + 0.08);
        gain.gain.setValueAtTime(0.5, this.ctx.currentTime);
        gain.gain.exponentialRampToValueAtTime(0.01, this.ctx.currentTime + 0.08);
        osc.connect(gain);
        gain.connect(this.ctx.destination);
        osc.start();
        osc.stop(this.ctx.currentTime + 0.08);
    }

    playCapture() {
        this.init();
        const osc = this.ctx.createOscillator();
        const gain = this.ctx.createGain();
        osc.type = 'triangle';
        osc.frequency.setValueAtTime(320, this.ctx.currentTime);
        osc.frequency.exponentialRampToValueAtTime(60, this.ctx.currentTime + 0.12);
        gain.gain.setValueAtTime(0.7, this.ctx.currentTime);
        gain.gain.exponentialRampToValueAtTime(0.01, this.ctx.currentTime + 0.12);
        osc.connect(gain);
        gain.connect(this.ctx.destination);
        osc.start();
        osc.stop(this.ctx.currentTime + 0.12);
    }

    playCheck() {
        this.init();
        const osc = this.ctx.createOscillator();
        const gain = this.ctx.createGain();
        osc.type = 'sine';
        osc.frequency.setValueAtTime(580, this.ctx.currentTime);
        osc.frequency.exponentialRampToValueAtTime(880, this.ctx.currentTime + 0.15);
        gain.gain.setValueAtTime(0.4, this.ctx.currentTime);
        gain.gain.exponentialRampToValueAtTime(0.01, this.ctx.currentTime + 0.18);
        osc.connect(gain);
        gain.connect(this.ctx.destination);
        osc.start();
        osc.stop(this.ctx.currentTime + 0.18);
    }
}

const audio = new ChessAudio();

const START_FEN = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

// Pure position & SAN helpers shared by game state and PV formatting
function uciToSq(coord) {
    const file = coord.charCodeAt(0) - 'a'.charCodeAt(0);
    const rank = 8 - parseInt(coord[1], 10);
    return rank * 8 + file;
}

function sqToUci(sq) {
    const file = String.fromCharCode('a'.charCodeAt(0) + (sq % 8));
    const rank = 8 - Math.floor(sq / 8);
    return `${file}${rank}`;
}

function isColorPiece(piece, color) {
    if (!piece) return false;
    return color === 'w' ? piece === piece.toUpperCase() : piece === piece.toLowerCase();
}

function clonePosition(pos) {
    return {
        board: pos.board.slice(),
        turn: pos.turn,
        castling: { ...pos.castling },
        enpassant: pos.enpassant,
        halfmove: pos.halfmove,
        fullmove: pos.fullmove
    };
}

function isRayClear(board, from, to) {
    const r1 = Math.floor(from / 8), c1 = from % 8;
    const r2 = Math.floor(to / 8), c2 = to % 8;
    const stepR = Math.sign(r2 - r1);
    const stepC = Math.sign(c2 - c1);
    let currR = r1 + stepR;
    let currC = c1 + stepC;

    while (currR !== r2 || currC !== c2) {
        if (board[currR * 8 + currC] !== null) return false;
        currR += stepR;
        currC += stepC;
    }
    return true;
}

function isSquareAttacked(pos, targetSq, byColor) {
    const tr = Math.floor(targetSq / 8);
    const tc = targetSq % 8;

    for (let sq = 0; sq < 64; sq++) {
        const p = pos.board[sq];
        if (!p) continue;
        const pColor = p === p.toUpperCase() ? 'w' : 'b';
        if (pColor !== byColor) continue;

        const r = Math.floor(sq / 8);
        const c = sq % 8;
        const dr = tr - r;
        const dc = tc - c;
        const type = p.toUpperCase();

        if (type === 'P') {
            const pawnDir = pColor === 'w' ? -1 : 1;
            if (dr === pawnDir && Math.abs(dc) === 1) return true;
        } else if (type === 'N') {
            if ((Math.abs(dr) === 1 && Math.abs(dc) === 2) || (Math.abs(dr) === 2 && Math.abs(dc) === 1)) return true;
        } else if (type === 'K') {
            if (Math.abs(dr) <= 1 && Math.abs(dc) <= 1) return true;
        } else if (type === 'R' || type === 'B' || type === 'Q') {
            const isRook = type === 'R' || type === 'Q';
            const isBishop = type === 'B' || type === 'Q';

            if (isRook && (dr === 0 || dc === 0) && isRayClear(pos.board, sq, targetSq)) return true;
            if (isBishop && Math.abs(dr) === Math.abs(dc) && isRayClear(pos.board, sq, targetSq)) return true;
        }
    }
    return false;
}

function findKingInCheck(pos, color) {
    const kingPiece = color === 'w' ? 'K' : 'k';
    const kingSq = pos.board.indexOf(kingPiece);
    if (kingSq === -1) return null;
    return isSquareAttacked(pos, kingSq, color === 'w' ? 'b' : 'w') ? kingSq : null;
}

function generateLegalMovesForPos(pos, from) {
    const moves = [];
    const piece = pos.board[from];
    if (!piece || !isColorPiece(piece, pos.turn)) return moves;

    const isWhite = pos.turn === 'w';
    const r = Math.floor(from / 8);
    const c = from % 8;
    const type = piece.toUpperCase();

    const addIfLegal = (to, isEnPassant = false) => {
        const origFrom = pos.board[from];
        const origTo = pos.board[to];
        let epPawnSq = null;
        let origEpPawn = null;

        if (isEnPassant) {
            epPawnSq = isWhite ? to + 8 : to - 8;
            origEpPawn = pos.board[epPawnSq];
            pos.board[epPawnSq] = null;
        }

        pos.board[to] = origFrom;
        pos.board[from] = null;

        const inCheck = findKingInCheck(pos, pos.turn) !== null;

        pos.board[from] = origFrom;
        pos.board[to] = origTo;
        if (isEnPassant) {
            pos.board[epPawnSq] = origEpPawn;
        }

        if (!inCheck) {
            moves.push({ from, to });
        }
    };

    if (type === 'P') {
        const dir = isWhite ? -1 : 1;
        const startRank = isWhite ? 6 : 1;
        const nextRank = r + dir;
        if (nextRank >= 0 && nextRank < 8) {
            const oneStep = nextRank * 8 + c;
            if (pos.board[oneStep] === null) {
                addIfLegal(oneStep);
                const twoRank = r + 2 * dir;
                if (r === startRank && twoRank >= 0 && twoRank < 8) {
                    const twoStep = twoRank * 8 + c;
                    if (pos.board[twoStep] === null) {
                        addIfLegal(twoStep);
                    }
                }
            }

            for (const dc of [-1, 1]) {
                const targetC = c + dc;
                if (targetC >= 0 && targetC < 8) {
                    const targetSq = nextRank * 8 + targetC;
                    const destPiece = pos.board[targetSq];
                    if (destPiece && !isColorPiece(destPiece, pos.turn)) {
                        addIfLegal(targetSq, false);
                    } else if (targetSq === pos.enpassant) {
                        addIfLegal(targetSq, true);
                    }
                }
            }
        }
    } else if (type === 'N') {
        const offsets = [
            [-2, -1], [-2, 1], [-1, -2], [-1, 2],
            [1, -2], [1, 2], [2, -1], [2, 1]
        ];
        for (const [dr, dc] of offsets) {
            const nr = r + dr, nc = c + dc;
            if (nr >= 0 && nr < 8 && nc >= 0 && nc < 8) {
                const to = nr * 8 + nc;
                if (!pos.board[to] || !isColorPiece(pos.board[to], pos.turn)) {
                    addIfLegal(to);
                }
            }
        }
    } else if (type === 'B' || type === 'R' || type === 'Q') {
        const dirs = [];
        if (type === 'B' || type === 'Q') dirs.push([-1, -1], [-1, 1], [1, -1], [1, 1]);
        if (type === 'R' || type === 'Q') dirs.push([-1, 0], [1, 0], [0, -1], [0, 1]);

        for (const [dr, dc] of dirs) {
            let nr = r + dr, nc = c + dc;
            while (nr >= 0 && nr < 8 && nc >= 0 && nc < 8) {
                const to = nr * 8 + nc;
                const targetPiece = pos.board[to];
                if (!targetPiece) {
                    addIfLegal(to);
                } else {
                    if (!isColorPiece(targetPiece, pos.turn)) addIfLegal(to);
                    break;
                }
                nr += dr;
                nc += dc;
            }
        }
    } else if (type === 'K') {
        for (let dr = -1; dr <= 1; dr++) {
            for (let dc = -1; dc <= 1; dc++) {
                if (dr === 0 && dc === 0) continue;
                const nr = r + dr, nc = c + dc;
                if (nr >= 0 && nr < 8 && nc >= 0 && nc < 8) {
                    const to = nr * 8 + nc;
                    if (!pos.board[to] || !isColorPiece(pos.board[to], pos.turn)) {
                        addIfLegal(to);
                    }
                }
            }
        }

        if (findKingInCheck(pos, pos.turn) === null) {
            if (isWhite && from === 60) {
                if (pos.castling.K && pos.board[63] === 'R' && !pos.board[61] && !pos.board[62] && !isSquareAttacked(pos, 61, 'b')) {
                    addIfLegal(62);
                }
                if (pos.castling.Q && pos.board[56] === 'R' && !pos.board[59] && !pos.board[58] && !pos.board[57] && !isSquareAttacked(pos, 59, 'b')) {
                    addIfLegal(58);
                }
            } else if (!isWhite && from === 4) {
                if (pos.castling.k && pos.board[7] === 'r' && !pos.board[5] && !pos.board[6] && !isSquareAttacked(pos, 5, 'w')) {
                    addIfLegal(6);
                }
                if (pos.castling.q && pos.board[0] === 'r' && !pos.board[3] && !pos.board[2] && !pos.board[1] && !isSquareAttacked(pos, 3, 'w')) {
                    addIfLegal(2);
                }
            }
        }
    }

    return moves;
}

function hasAnyLegalMove(pos) {
    for (let sq = 0; sq < 64; sq++) {
        if (pos.board[sq] && isColorPiece(pos.board[sq], pos.turn)) {
            if (generateLegalMovesForPos(pos, sq).length > 0) {
                return true;
            }
        }
    }
    return false;
}

function applyMoveToPos(pos, from, to, promo = null) {
    const piece = pos.board[from];
    const target = pos.board[to];
    const isEnPassant = (piece === 'P' || piece === 'p') && to === pos.enpassant;
    const isCapture = !!target || isEnPassant;

    if (isEnPassant) {
        const epPawnSq = pos.turn === 'w' ? to + 8 : to - 8;
        pos.board[epPawnSq] = null;
    }

    pos.board[to] = promo ? (pos.turn === 'w' ? promo.toUpperCase() : promo.toLowerCase()) : piece;
    pos.board[from] = null;

    if (piece === 'K' && from === 60) {
        if (to === 62) { pos.board[61] = 'R'; pos.board[63] = null; }
        if (to === 58) { pos.board[59] = 'R'; pos.board[56] = null; }
    }
    if (piece === 'k' && from === 4) {
        if (to === 6) { pos.board[5] = 'r'; pos.board[7] = null; }
        if (to === 2) { pos.board[3] = 'r'; pos.board[0] = null; }
    }

    if (piece === 'K') { pos.castling.K = false; pos.castling.Q = false; }
    if (piece === 'k') { pos.castling.k = false; pos.castling.q = false; }
    if (from === 63 || to === 63) pos.castling.K = false;
    if (from === 56 || to === 56) pos.castling.Q = false;
    if (from === 7 || to === 7) pos.castling.k = false;
    if (from === 0 || to === 0) pos.castling.q = false;

    if (piece === 'P' && from - to === 16) {
        pos.enpassant = from - 8;
    } else if (piece === 'p' && to - from === 16) {
        pos.enpassant = from + 8;
    } else {
        pos.enpassant = null;
    }

    if (piece === 'P' || piece === 'p' || isCapture) {
        pos.halfmove = 0;
    } else {
        pos.halfmove++;
    }
    if (pos.turn === 'b') {
        pos.fullmove++;
    }

    pos.turn = pos.turn === 'w' ? 'b' : 'w';
    return { isCapture, isEnPassant };
}

function moveToSan(pos, from, to, promo = null) {
    const piece = pos.board[from];
    if (!piece) return `${sqToUci(from)}${sqToUci(to)}${promo || ''}`;

    const type = piece.toUpperCase();
    const destSquare = sqToUci(to);
    const isEnPassant = type === 'P' && to === pos.enpassant;
    const isCapture = !!pos.board[to] || isEnPassant;

    let san = '';
    if (type === 'K' && Math.abs((from % 8) - (to % 8)) === 2) {
        san = (to % 8) === 6 ? 'O-O' : 'O-O-O';
    } else if (type === 'P') {
        if (isCapture) {
            const fromFile = String.fromCharCode('a'.charCodeAt(0) + (from % 8));
            san = `${fromFile}x${destSquare}`;
        } else {
            san = destSquare;
        }
        if (promo) {
            san += `=${promo.toUpperCase()}`;
        }
    } else {
        let disambig = '';
        if (type !== 'K') {
            const others = [];
            for (let sq = 0; sq < 64; sq++) {
                if (sq === from) continue;
                const p = pos.board[sq];
                if (p === piece) {
                    const legal = generateLegalMovesForPos(pos, sq);
                    if (legal.some(m => m.to === to)) {
                        others.push(sq);
                    }
                }
            }
            if (others.length > 0) {
                const fromFile = from % 8;
                const fromRank = Math.floor(from / 8);
                const sameFile = others.some(sq => (sq % 8) === fromFile);
                const sameRank = others.some(sq => Math.floor(sq / 8) === fromRank);
                const fileChar = String.fromCharCode('a'.charCodeAt(0) + fromFile);
                const rankChar = String(8 - fromRank);
                if (!sameFile) {
                    disambig = fileChar;
                } else if (!sameRank) {
                    disambig = rankChar;
                } else {
                    disambig = `${fileChar}${rankChar}`;
                }
            }
        }
        san = `${type}${disambig}${isCapture ? 'x' : ''}${destSquare}`;
    }

    const nextPos = clonePosition(pos);
    applyMoveToPos(nextPos, from, to, promo);
    if (findKingInCheck(nextPos, nextPos.turn) !== null) {
        san += hasAnyLegalMove(nextPos) ? '+' : '#';
    }

    return san;
}

function uciPvToSan(pos, pvStr) {
    if (!pvStr || !pvStr.trim()) return '-';
    const tokens = pvStr.trim().split(/\s+/);
    const simPos = clonePosition(pos);
    const parts = [];

    for (let i = 0; i < tokens.length; i++) {
        const uci = tokens[i];
        if (uci.length < 4) {
            parts.push(tokens.slice(i).join(' '));
            break;
        }
        const from = uciToSq(uci.substring(0, 2));
        const to = uciToSq(uci.substring(2, 4));
        const promo = uci.length >= 5 ? uci[4].toLowerCase() : null;

        if (from < 0 || from >= 64 || to < 0 || to >= 64) {
            parts.push(tokens.slice(i).join(' '));
            break;
        }
        const legal = generateLegalMovesForPos(simPos, from);
        if (!legal.some(m => m.to === to)) {
            parts.push(tokens.slice(i).join(' '));
            break;
        }

        const san = moveToSan(simPos, from, to, promo);
        if (simPos.turn === 'w') {
            parts.push(`${simPos.fullmove}. ${san}`);
        } else if (i === 0) {
            parts.push(`${simPos.fullmove}... ${san}`);
        } else {
            parts.push(san);
        }

        applyMoveToPos(simPos, from, to, promo);
    }

    return parts.join(' ');
}


class ChessGame {
    constructor() {
        this.board = new Array(64).fill(null);
        this.turn = 'w';
        this.castling = { K: true, Q: true, k: true, q: true };
        this.enpassant = null; // square index 0..63
        this.halfmove = 0;
        this.fullmove = 1;
        this.history = []; // { fen, uci, san, from, to, piece }
        this.selectedSquare = null;
        this.legalMovesForSelected = [];
        this.lastMove = null; // { from, to }
        this.isFlipped = false;
        this.gameMode = 'bot'; // 'bot', 'pvp', 'bvb'
        this.playerSide = 'w';
        this.botDepth = 6;
        this.pendingPromotion = null;

        // Concurrency & async request tracking
        this.analysisSeqId = 0;
        this.analysisAbortController = null;
        this.botSeqId = 0;
        this.botAbortController = null;
        this.botThinking = false;
        this.bvbTimer = null;

        // Eval bar state
        this.lastScore = 0;
        this.lastEvalAdvantage = 'white';

        // Pointer drag-and-drop state
        this.dragState = null;
        this.suppressNextClick = false;

        this.initDOM();
        this.loadFen(START_FEN);
        this.render();
        this.attachEvents();
        this.triggerAnalysis();
    }

    initDOM() {
        this.boardEl = document.getElementById('chessboard');
        this.evalBarContainerEl = document.getElementById('evalBarContainer');
        this.evalWhiteEl = document.getElementById('evalWhite');
        this.evalBadgeEl = document.getElementById('evalBadge');
        this.statusTextEl = document.getElementById('statusText');
        this.fenInputEl = document.getElementById('fenInput');
        this.historyBodyEl = document.getElementById('historyBody');
        this.moveCountEl = document.getElementById('moveCount');
        this.depthValEl = document.getElementById('depthVal');
        this.depthSliderEl = document.getElementById('depthSlider');
        this.analysisDepthEl = document.getElementById('analysisDepth');
        this.promoModalEl = document.getElementById('promotionModal');

        this.pvScores = [
            document.getElementById('pvScore1'),
            document.getElementById('pvScore2'),
            document.getElementById('pvScore3')
        ];
        this.pvMoves = [
            document.getElementById('pvMoves1'),
            document.getElementById('pvMoves2'),
            document.getElementById('pvMoves3')
        ];
    }

    attachEvents() {
        document.getElementById('newGameBtn').onclick = () => this.resetGame();
        document.getElementById('undoBtn').onclick = () => this.undoMove();
        document.getElementById('flipBtn').onclick = () => this.toggleFlip();
        document.getElementById('copyFenBtn').onclick = () => {
            navigator.clipboard.writeText(this.fenInputEl.value);
            this.statusTextEl.innerText = "FEN copied to clipboard!";
            setTimeout(() => this.updateStatus(), 1500);
        };

        // Modes
        document.getElementById('modeBtnBot').onclick = (e) => this.setMode('bot', e.currentTarget);
        document.getElementById('modeBtnPvp').onclick = (e) => this.setMode('pvp', e.currentTarget);
        document.getElementById('modeBtnBvb').onclick = (e) => this.setMode('bvb', e.currentTarget);

        // Play As (Human side)
        document.getElementById('playWhiteBtn').onclick = () => this.setPlayerSide('w');
        document.getElementById('playBlackBtn').onclick = () => this.setPlayerSide('b');

        // Depth Slider
        this.depthSliderEl.oninput = (e) => {
            this.botDepth = parseInt(e.target.value, 10);
            this.depthValEl.innerText = this.botDepth;
        };

        // Promotion Buttons
        document.querySelectorAll('.promo-btn').forEach(btn => {
            btn.onclick = () => {
                if (this.pendingPromotion) {
                    const piece = btn.dataset.piece;
                    const { from, to } = this.pendingPromotion;
                    this.promoModalEl.style.display = 'none';
                    this.pendingPromotion = null;
                    this.executeMove(from, to, piece);
                }
            };
        });

        // Hint button
        document.getElementById('hintBtn').onclick = () => {
            if (this.botThinking) return;
            if (this.gameMode === 'bot' && this.turn !== this.playerSide) return;
            if (this.pvMoves[0] && this.pvMoves[0].dataset.firstMove) {
                const uci = this.pvMoves[0].dataset.firstMove;
                const from = this.uciToSq(uci.substring(0, 2));
                const to = this.uciToSq(uci.substring(2, 4));
                this.selectedSquare = from;
                this.legalMovesForSelected = this.generateLegalMoves(from);
                this.render();
                const toSqEl = document.querySelector(`.square[data-sq="${to}"]`);
                if (toSqEl) toSqEl.classList.add('highlight');
            }
        };

        // Global pointer move/up/cancel for smooth drag-and-drop
        window.addEventListener('pointermove', (e) => this.handlePointerMove(e));
        window.addEventListener('pointerup', (e) => this.handlePointerUp(e));
        window.addEventListener('pointercancel', () => this.cancelDrag());
    }

    canHumanInteract() {
        if (this.botThinking || this.pendingPromotion) return false;
        if (this.gameMode === 'bvb') return false;
        if (this.gameMode === 'bot' && this.turn !== this.playerSide) return false;
        return this.hasAnyLegalMoves();
    }

    cancelBotPending() {
        this.botSeqId++;
        if (this.botAbortController) {
            this.botAbortController.abort();
            this.botAbortController = null;
        }
        if (this.bvbTimer) {
            clearTimeout(this.bvbTimer);
            this.bvbTimer = null;
        }
        this.botThinking = false;
    }

    cancelPendingPromotion() {
        this.pendingPromotion = null;
        this.promoModalEl.style.display = 'none';
    }

    setPlayerSide(side) {
        this.cancelBotPending();
        this.cancelPendingPromotion();
        this.cancelDrag();
        this.playerSide = side;
        document.getElementById('playWhiteBtn').classList.toggle('active', side === 'w');
        document.getElementById('playBlackBtn').classList.toggle('active', side === 'b');

        const shouldFlip = side === 'b';
        if (this.isFlipped !== shouldFlip) {
            this.isFlipped = shouldFlip;
            this.render();
            this.updateEvalBarAppearance();
        }
        this.updateStatus();

        if (this.gameMode === 'bot' && this.turn !== this.playerSide && this.hasAnyLegalMoves()) {
            this.triggerBotMove();
        }
    }

    setMode(mode, targetBtn) {
        this.cancelBotPending();
        this.cancelPendingPromotion();
        this.cancelDrag();
        this.gameMode = mode;
        document.querySelectorAll('.mode-btn').forEach(b => b.classList.remove('active'));
        targetBtn.classList.add('active');
        this.selectedSquare = null;
        this.legalMovesForSelected = [];
        this.render();
        this.updateStatus();

        if (mode === 'bvb' && this.hasAnyLegalMoves()) {
            this.triggerBotMove();
        } else if (mode === 'bot' && this.turn !== this.playerSide && this.hasAnyLegalMoves()) {
            this.triggerBotMove();
        }
    }

    toggleFlip() {
        this.isFlipped = !this.isFlipped;
        this.render();
        this.updateEvalBarAppearance();
    }

    resetGame() {
        this.cancelBotPending();
        this.cancelPendingPromotion();
        this.cancelDrag();
        this.loadFen(START_FEN);
        this.history = [];
        this.lastMove = null;
        this.selectedSquare = null;
        this.legalMovesForSelected = [];
        this.render();
        this.updateHistoryTable();

        // Explicitly reset TT only on New Game
        fetch('/api/new_game', { method: 'POST' }).catch(() => {});

        this.triggerAnalysis();
        if (this.gameMode === 'bot' && this.turn !== this.playerSide) {
            this.triggerBotMove();
        } else if (this.gameMode === 'bvb') {
            this.triggerBotMove();
        }
    }

    undoMove() {
        if (this.history.length === 0) return;
        this.cancelBotPending();
        this.cancelPendingPromotion();
        this.cancelDrag();

        if (this.gameMode === 'bot') {
            if (this.turn !== this.playerSide) {
                // Human just moved (bot was thinking or game ended): undo 1 move back to human's turn
                const prev = this.history.pop();
                this.loadFen(prev.fen);
            } else if (this.history.length >= 2) {
                // Human's turn after bot replied: undo both bot and human moves
                this.history.pop();
                const prev = this.history.pop();
                this.loadFen(prev.fen);
            } else {
                // Only 1 bot move exists (human plays Black and hasn't moved yet)
                this.updateStatus();
                return;
            }
        } else {
            const prev = this.history.pop();
            this.loadFen(prev.fen);
        }

        const top = this.history[this.history.length - 1];
        this.lastMove = top ? { from: top.from, to: top.to } : null;
        this.selectedSquare = null;
        this.legalMovesForSelected = [];
        this.render();
        this.updateHistoryTable();
        this.triggerAnalysis();
    }

    loadFen(fen) {
        const parts = fen.split(' ');
        const rows = parts[0].split('/');
        this.board.fill(null);

        for (let r = 0; r < 8; r++) {
            let col = 0;
            for (const char of rows[r]) {
                if (char >= '1' && char <= '8') {
                    col += parseInt(char, 10);
                } else {
                    this.board[r * 8 + col] = char;
                    col++;
                }
            }
        }

        this.turn = parts[1] || 'w';
        this.castling = {
            K: parts[2]?.includes('K') || false,
            Q: parts[2]?.includes('Q') || false,
            k: parts[2]?.includes('k') || false,
            q: parts[2]?.includes('q') || false
        };
        this.enpassant = parts[3] && parts[3] !== '-' ? this.uciToSq(parts[3]) : null;
        this.halfmove = parseInt(parts[4], 10) || 0;
        this.fullmove = parseInt(parts[5], 10) || 1;

        this.fenInputEl.value = this.getFen();
        this.updateStatus();
    }

    getFen() {
        let fen = '';
        for (let r = 0; r < 8; r++) {
            let empty = 0;
            for (let c = 0; c < 8; c++) {
                const piece = this.board[r * 8 + c];
                if (!piece) {
                    empty++;
                } else {
                    if (empty > 0) {
                        fen += empty;
                        empty = 0;
                    }
                    fen += piece;
                }
            }
            if (empty > 0) fen += empty;
            if (r < 7) fen += '/';
        }

        fen += ` ${this.turn} `;
        let castlingStr = '';
        if (this.castling.K) castlingStr += 'K';
        if (this.castling.Q) castlingStr += 'Q';
        if (this.castling.k) castlingStr += 'k';
        if (this.castling.q) castlingStr += 'q';
        fen += castlingStr || '-';

        fen += ` ${this.enpassant !== null ? this.sqToUci(this.enpassant) : '-'} `;
        fen += `${this.halfmove} ${this.fullmove}`;
        return fen;
    }

    uciToSq(coord) {
        return uciToSq(coord);
    }

    sqToUci(sq) {
        return sqToUci(sq);
    }

    render() {
        this.boardEl.innerHTML = '';
        const inCheckSq = this.findKingInCheck(this.turn);

        for (let idx = 0; idx < 64; idx++) {
            const sq = this.isFlipped ? 63 - idx : idx;
            const r = Math.floor(sq / 8);
            const c = sq % 8;
            const isLight = (r + c) % 2 === 0;

            const sqEl = document.createElement('div');
            sqEl.className = `square ${isLight ? 'light' : 'dark'}`;
            sqEl.dataset.sq = sq;

            if (this.lastMove && (sq === this.lastMove.from || sq === this.lastMove.to)) {
                sqEl.classList.add('highlight');
            }
            if (this.selectedSquare === sq) {
                sqEl.classList.add('selected');
            }
            if (sq === inCheckSq) {
                sqEl.classList.add('check');
            }

            // Coordinates
            if (this.isFlipped ? c === 7 : c === 0) {
                const rankEl = document.createElement('span');
                rankEl.className = 'coord coord-rank';
                rankEl.innerText = 8 - r;
                sqEl.appendChild(rankEl);
            }
            if (this.isFlipped ? r === 0 : r === 7) {
                const fileEl = document.createElement('span');
                fileEl.className = 'coord coord-file';
                fileEl.innerText = String.fromCharCode('a'.charCodeAt(0) + c);
                sqEl.appendChild(fileEl);
            }

            // Piece
            const piece = this.board[sq];
            if (piece) {
                const pieceEl = document.createElement('div');
                pieceEl.className = 'piece';
                pieceEl.innerHTML = PIECE_SVGS[piece] || '';
                sqEl.appendChild(pieceEl);
            }

            // Legal move indicators
            if (this.selectedSquare !== null) {
                const isTarget = this.legalMovesForSelected.some(m => m.to === sq);
                if (isTarget) {
                    const indicator = document.createElement('div');
                    indicator.className = piece ? 'capture-ring' : 'move-dot';
                    sqEl.appendChild(indicator);
                }
            }

            sqEl.addEventListener('pointerdown', (e) => this.handlePointerDown(e, sq));
            sqEl.onclick = () => this.handleSquareClick(sq);
            this.boardEl.appendChild(sqEl);
        }
    }

    handlePointerDown(e, sq) {
        if (e.button !== 0) return;
        if (!this.canHumanInteract()) return;

        const piece = this.board[sq];
        if (!piece || !this.isOwnPiece(piece)) return;

        // If user already selected another piece and is clicking an enemy/capture, don't start drag
        this.dragState = {
            fromSq: sq,
            startX: e.clientX,
            startY: e.clientY,
            isDragging: false,
            ghostEl: null,
            piece,
            hoveredSq: null
        };
    }

    handlePointerMove(e) {
        if (!this.dragState) return;
        const dist = Math.hypot(e.clientX - this.dragState.startX, e.clientY - this.dragState.startY);

        if (!this.dragState.isDragging) {
            if (dist < 5) return;
            this.dragState.isDragging = true;

            this.selectedSquare = this.dragState.fromSq;
            this.legalMovesForSelected = this.generateLegalMoves(this.dragState.fromSq);
            this.render();

            const srcSqEl = this.boardEl.querySelector(`.square[data-sq="${this.dragState.fromSq}"]`);
            if (srcSqEl) srcSqEl.classList.add('drag-source');

            const rect = srcSqEl ? srcSqEl.getBoundingClientRect() : { width: 64, height: 64 };
            const ghost = document.createElement('div');
            ghost.className = 'drag-ghost';
            ghost.style.width = `${rect.width * 0.95}px`;
            ghost.style.height = `${rect.height * 0.95}px`;
            ghost.innerHTML = PIECE_SVGS[this.dragState.piece] || '';
            document.body.appendChild(ghost);
            this.dragState.ghostEl = ghost;
        }

        if (this.dragState.ghostEl) {
            this.dragState.ghostEl.style.left = `${e.clientX}px`;
            this.dragState.ghostEl.style.top = `${e.clientY}px`;
        }

        const targetSq = this.getSquareAtPoint(e.clientX, e.clientY);
        if (targetSq !== this.dragState.hoveredSq) {
            if (this.dragState.hoveredSq !== null) {
                const prevEl = this.boardEl.querySelector(`.square[data-sq="${this.dragState.hoveredSq}"]`);
                if (prevEl) prevEl.classList.remove('drag-over');
            }
            this.dragState.hoveredSq = targetSq;
            if (targetSq !== null && this.legalMovesForSelected.some(m => m.to === targetSq)) {
                const nextEl = this.boardEl.querySelector(`.square[data-sq="${targetSq}"]`);
                if (nextEl) nextEl.classList.add('drag-over');
            }
        }
    }

    handlePointerUp(e) {
        if (!this.dragState) return;
        const drag = this.dragState;
        this.dragState = null;

        if (!drag.isDragging) {
            return;
        }

        if (drag.ghostEl) drag.ghostEl.remove();
        this.suppressNextClick = true;
        setTimeout(() => { this.suppressNextClick = false; }, 0);

        const targetSq = this.getSquareAtPoint(e.clientX, e.clientY);
        if (targetSq !== null && targetSq !== drag.fromSq) {
            const move = this.legalMovesForSelected.find(m => m.to === targetSq);
            if (move) {
                const piece = this.board[drag.fromSq];
                const destRank = Math.floor(targetSq / 8);
                if ((piece === 'P' && destRank === 0) || (piece === 'p' && destRank === 7)) {
                    this.pendingPromotion = { from: drag.fromSq, to: targetSq };
                    this.promoModalEl.style.display = 'flex';
                    this.render();
                    return;
                }
                this.executeMove(drag.fromSq, targetSq);
                return;
            }
        }

        this.render();
    }

    cancelDrag() {
        if (!this.dragState) return;
        if (this.dragState.ghostEl) this.dragState.ghostEl.remove();
        this.dragState = null;
        this.render();
    }

    getSquareAtPoint(clientX, clientY) {
        const rect = this.boardEl.getBoundingClientRect();
        if (clientX < rect.left || clientX >= rect.right || clientY < rect.top || clientY >= rect.bottom) {
            return null;
        }
        const col = Math.floor(((clientX - rect.left) / rect.width) * 8);
        const row = Math.floor(((clientY - rect.top) / rect.height) * 8);
        if (col < 0 || col > 7 || row < 0 || row > 7) return null;
        const idx = row * 8 + col;
        return this.isFlipped ? 63 - idx : idx;
    }

    handleSquareClick(sq) {
        if (this.suppressNextClick) {
            this.suppressNextClick = false;
            return;
        }
        if (!this.canHumanInteract()) return;

        if (this.selectedSquare === sq) {
            this.selectedSquare = null;
            this.legalMovesForSelected = [];
            this.render();
            return;
        }

        if (this.selectedSquare !== null) {
            const move = this.legalMovesForSelected.find(m => m.to === sq);
            if (move) {
                const piece = this.board[this.selectedSquare];
                const destRank = Math.floor(sq / 8);
                if ((piece === 'P' && destRank === 0) || (piece === 'p' && destRank === 7)) {
                    this.pendingPromotion = { from: this.selectedSquare, to: sq };
                    this.promoModalEl.style.display = 'flex';
                    return;
                }
                this.executeMove(this.selectedSquare, sq);
                return;
            }
        }

        const piece = this.board[sq];
        if (piece && this.isOwnPiece(piece)) {
            this.selectedSquare = sq;
            this.legalMovesForSelected = this.generateLegalMoves(sq);
            this.render();
        } else {
            this.selectedSquare = null;
            this.legalMovesForSelected = [];
            this.render();
        }
    }

    isOwnPiece(piece) {
        return isColorPiece(piece, this.turn);
    }

    executeMove(from, to, promo = null) {
        const piece = this.board[from];
        const beforeFen = this.getFen();
        const san = moveToSan(this, from, to, promo);
        const { isCapture } = applyMoveToPos(this, from, to, promo);

        this.lastMove = { from, to };
        this.selectedSquare = null;
        this.legalMovesForSelected = [];

        const inCheck = this.findKingInCheck(this.turn) !== null;
        if (inCheck) {
            audio.playCheck();
        } else if (isCapture) {
            audio.playCapture();
        } else {
            audio.playMove();
        }

        const uciMove = `${this.sqToUci(from)}${this.sqToUci(to)}${promo || ''}`;
        this.history.push({
            fen: beforeFen,
            uci: uciMove,
            san,
            from,
            to,
            piece
        });

        this.fenInputEl.value = this.getFen();
        this.render();
        this.updateHistoryTable();
        this.updateStatus();
        this.triggerAnalysis();

        if (!this.hasAnyLegalMoves()) {
            return;
        }

        if (this.gameMode === 'bot' && this.turn !== this.playerSide) {
            this.triggerBotMove();
        } else if (this.gameMode === 'bvb') {
            this.bvbTimer = setTimeout(() => {
                this.bvbTimer = null;
                if (this.gameMode === 'bvb' && this.hasAnyLegalMoves()) {
                    this.triggerBotMove();
                }
            }, 400);
        }
    }

    triggerAnalysis() {
        const seqId = ++this.analysisSeqId;
        if (this.analysisAbortController) {
            this.analysisAbortController.abort();
        }
        this.analysisAbortController = new AbortController();
        const fen = this.getFen();
        const posSnapshot = clonePosition(this);

        fetch('/api/analyze', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ fen, depth: 18, movetime: 2000 }),
            signal: this.analysisAbortController.signal
        })
        .then(res => res.json())
        .then(data => {
            if (seqId !== this.analysisSeqId) return;
            if (data) {
                this.updateAnalysisUI(data, posSnapshot);
            }
        })
        .catch(err => {
            if (err && err.name === 'AbortError') return;
        });
    }

    updateEvalBarAppearance() {
        if (this.evalBarContainerEl) {
            this.evalBarContainerEl.classList.toggle('flipped', this.isFlipped);
        }
        const whiteAdvantage = this.lastEvalAdvantage === 'white';
        const placeAtBottom = this.isFlipped ? !whiteAdvantage : whiteAdvantage;
        this.evalBadgeEl.classList.toggle('pos-bottom', placeAtBottom);
        this.evalBadgeEl.classList.toggle('pos-top', !placeAtBottom);
        this.evalBadgeEl.classList.toggle('adv-white', whiteAdvantage);
        this.evalBadgeEl.classList.toggle('adv-black', !whiteAdvantage);
    }

    updateAnalysisUI(data, posSnapshot = null) {
        const pos = posSnapshot || this;
        const hasLegal = hasAnyLegalMove(pos);
        const inCheck = findKingInCheck(pos, pos.turn) !== null;

        let score = data.score || 0;
        let evalStr = data.eval || "+0.0";
        let whitePct = 50;

        if (!hasLegal) {
            if (inCheck) {
                const whiteWon = pos.turn === 'b';
                evalStr = whiteWon ? '+M0' : '-M0';
                score = whiteWon ? 49000 : -49000;
                whitePct = whiteWon ? 100 : 0;
            } else {
                evalStr = '0.0';
                score = 0;
                whitePct = 50;
            }
        } else if (data.score_type === 'mate' || (data.mate !== null && data.mate !== undefined)) {
            const mateVal = (data.mate !== null && data.mate !== undefined) ? data.mate : score;
            const whiteMate = mateVal > 0;
            whitePct = whiteMate ? 98 : 2;
            score = whiteMate ? Math.max(10000, score) : Math.min(-10000, score);
            evalStr = data.eval || (whiteMate ? `+M${Math.abs(mateVal)}` : `-M${Math.abs(mateVal)}`);
        } else {
            const winProb = typeof data.win_prob === 'number'
                ? data.win_prob
                : 1.0 / (1.0 + Math.pow(10, -score / 400.0));
            whitePct = Math.round(winProb * 100);
            whitePct = Math.max(5, Math.min(95, whitePct));
        }

        this.lastScore = score;
        this.lastEvalAdvantage = score >= 0 ? 'white' : 'black';
        this.evalBadgeEl.innerText = evalStr;
        this.evalWhiteEl.style.height = `${whitePct}%`;
        this.updateEvalBarAppearance();

        // Update MultiPV lines with full numbered SAN continuations
        this.analysisDepthEl.innerText = `Depth ${data.depth || 0}`;
        const lines = Array.isArray(data.lines) ? data.lines : [];

        for (let i = 0; i < 3; i++) {
            const line = lines[i];
            if (line) {
                const s = line.white_score !== undefined ? line.white_score : line.score;
                let formattedScore;
                if (line.score_type === 'mate') {
                    if (line.eval) {
                        formattedScore = line.eval;
                    } else {
                        const m = line.mate !== undefined && line.mate !== null ? line.mate : s;
                        formattedScore = m > 0 ? `+M${m}` : `-M${Math.abs(m)}`;
                    }
                } else {
                    formattedScore = line.eval || `${s >= 0 ? '+' : ''}${(s / 100).toFixed(1)}`;
                }

                this.pvScores[i].innerText = formattedScore;
                this.pvScores[i].className = `pv-score ${s >= 0 ? 'score-pos' : 'score-neg'}`;

                const pvTokens = (line.pv || '').trim().split(/\s+/).filter(Boolean);
                this.pvMoves[i].innerText = uciPvToSan(pos, line.pv || '');
                this.pvMoves[i].dataset.firstMove = pvTokens[0] || '';
            } else {
                this.pvScores[i].innerText = '-';
                this.pvScores[i].className = 'pv-score';
                this.pvMoves[i].innerText = hasLegal ? '-' : (inCheck ? 'Checkmate' : 'Stalemate');
                this.pvMoves[i].dataset.firstMove = '';
            }
        }
    }

    triggerBotMove() {
        if (this.botThinking) return;
        if (!this.hasAnyLegalMoves()) {
            this.updateStatus();
            return;
        }

        const seqId = ++this.botSeqId;
        if (this.botAbortController) {
            this.botAbortController.abort();
        }
        this.botAbortController = new AbortController();
        this.botThinking = true;
        this.statusTextEl.innerText = "Engine thinking...";

        const fen = this.getFen();
        fetch('/api/bot_move', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ fen, depth: this.botDepth }),
            signal: this.botAbortController.signal
        })
        .then(res => res.json())
        .then(data => {
            if (seqId !== this.botSeqId) return;
            this.botThinking = false;
            this.botAbortController = null;

            if (!data || data.game_over || !data.move || data.move === '(none)' || data.move === '0000') {
                this.updateStatus();
                return;
            }

            const uci = data.move;
            const from = this.uciToSq(uci.substring(0, 2));
            const to = this.uciToSq(uci.substring(2, 4));
            const promo = uci.length >= 5 ? uci[4] : null;
            this.executeMove(from, to, promo);
        })
        .catch(err => {
            if (err && err.name === 'AbortError') return;
            if (seqId !== this.botSeqId) return;
            this.botThinking = false;
            this.botAbortController = null;
            this.updateStatus();
        });
    }

    findKingInCheck(color) {
        return findKingInCheck(this, color);
    }

    isSquareAttacked(targetSq, byColor) {
        return isSquareAttacked(this, targetSq, byColor);
    }

    isRayClear(from, to) {
        return isRayClear(this.board, from, to);
    }

    generateLegalMoves(from) {
        return generateLegalMovesForPos(this, from);
    }

    hasAnyLegalMoves() {
        return hasAnyLegalMove(this);
    }

    updateStatus() {
        const inCheck = this.findKingInCheck(this.turn) !== null;
        const hasLegal = this.hasAnyLegalMoves();

        if (!hasLegal) {
            if (inCheck) {
                this.statusTextEl.innerText = `Checkmate! ${this.turn === 'w' ? 'Black' : 'White'} wins!`;
            } else {
                this.statusTextEl.innerText = "Draw by Stalemate!";
            }
        } else if (this.botThinking) {
            this.statusTextEl.innerText = "Engine thinking...";
        } else if (inCheck) {
            this.statusTextEl.innerText = `${this.turn === 'w' ? 'White' : 'Black'} is in Check!`;
        } else {
            this.statusTextEl.innerText = `${this.turn === 'w' ? 'White' : 'Black'} to move`;
        }
    }

    updateHistoryTable() {
        this.historyBodyEl.innerHTML = '';
        this.moveCountEl.innerText = `${this.history.length} moves`;

        for (let i = 0; i < this.history.length; i += 2) {
            const tr = document.createElement('tr');
            const moveNum = Math.floor(i / 2) + 1;
            const wMove = this.history[i] ? (this.history[i].san || this.history[i].uci) : '';
            const bMove = this.history[i + 1] ? (this.history[i + 1].san || this.history[i + 1].uci) : '';

            tr.innerHTML = `
                <td>${moveNum}.</td>
                <td>${wMove}</td>
                <td>${bMove}</td>
            `;
            this.historyBodyEl.appendChild(tr);
        }

        const wrapper = document.querySelector('.history-list-wrapper');
        if (wrapper) wrapper.scrollTop = wrapper.scrollHeight;
    }
}

// Bootstrap
window.addEventListener('DOMContentLoaded', () => {
    window.game = new ChessGame();
});
