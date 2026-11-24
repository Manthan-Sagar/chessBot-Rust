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

// Initial starting position
const START_FEN = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

class ChessGame {
    constructor() {
        this.board = new Array(64).fill(null);
        this.turn = 'w';
        this.castling = { K: true, Q: true, k: true, q: true };
        this.enpassant = null; // square index 0..63
        this.halfmove = 0;
        this.fullmove = 1;
        this.history = []; // { fen, moveUci, san, captured }
        this.selectedSquare = null;
        this.legalMovesForSelected = [];
        this.lastMove = null; // { from, to }
        this.isFlipped = false;
        this.gameMode = 'bot'; // 'bot', 'pvp', 'bvb'
        this.playerSide = 'w';
        this.botDepth = 6;
        this.pendingPromotion = null;
        this.isAnalyzing = false;
        this.botThinking = false;

        this.initDOM();
        this.loadFen(START_FEN);
        this.attachEvents();
        this.triggerAnalysis();
    }

    initDOM() {
        this.boardEl = document.getElementById('chessboard');
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
        document.getElementById('modeBtnBot').onclick = (e) => this.setMode('bot', e.target);
        document.getElementById('modeBtnPvp').onclick = (e) => this.setMode('pvp', e.target);
        document.getElementById('modeBtnBvb').onclick = (e) => this.setMode('bvb', e.target);

        // Player side
        document.getElementById('playWhiteBtn').onclick = (e) => {
            this.playerSide = 'w';
            e.target.classList.add('active');
            document.getElementById('playBlackBtn').classList.remove('active');
            if (this.isFlipped) this.toggleFlip();
            if (this.turn !== this.playerSide && this.gameMode === 'bot') this.triggerBotMove();
        };
        document.getElementById('playBlackBtn').onclick = (e) => {
            this.playerSide = 'b';
            e.target.classList.add('active');
            document.getElementById('playWhiteBtn').classList.remove('active');
            if (!this.isFlipped) this.toggleFlip();
            if (this.turn !== this.playerSide && this.gameMode === 'bot') this.triggerBotMove();
        };

        // Depth Slider
        this.depthSliderEl.oninput = (e) => {
            this.botDepth = parseInt(e.target.value);
            this.depthValEl.innerText = this.botDepth;
        };

        // Promotion Buttons
        document.querySelectorAll('.promo-btn').forEach(btn => {
            btn.onclick = () => {
                if (this.pendingPromotion) {
                    const piece = btn.dataset.piece;
                    this.executeMove(this.pendingPromotion.from, this.pendingPromotion.to, piece);
                    this.promoModalEl.style.display = 'none';
                    this.pendingPromotion = null;
                }
            };
        });

        // Hint button
        document.getElementById('hintBtn').onclick = () => {
            if (this.pvMoves[0] && this.pvMoves[0].dataset.firstMove) {
                const uci = this.pvMoves[0].dataset.firstMove;
                const from = this.uciToSq(uci.substring(0, 2));
                const to = this.uciToSq(uci.substring(2, 4));
                this.selectedSquare = from;
                this.render();
                // Highlight hint
                const toSqEl = document.querySelector(`.square[data-sq="${to}"]`);
                if (toSqEl) toSqEl.classList.add('highlight');
            }
        };
    }

    setMode(mode, targetBtn) {
        this.gameMode = mode;
        document.querySelectorAll('.mode-btn').forEach(b => b.classList.remove('active'));
        targetBtn.classList.add('active');
        if (mode === 'bvb') {
            this.triggerBotMove();
        }
    }

    toggleFlip() {
        this.isFlipped = !this.isFlipped;
        this.render();
    }

    resetGame() {
        this.loadFen(START_FEN);
        this.history = [];
        this.lastMove = null;
        this.render();
        this.updateHistoryTable();
        this.triggerAnalysis();
        if (this.gameMode === 'bot' && this.turn !== this.playerSide) {
            this.triggerBotMove();
        } else if (this.gameMode === 'bvb') {
            this.triggerBotMove();
        }
    }

    undoMove() {
        if (this.history.length === 0) return;
        // In bot mode, undo both bot move and user move
        if (this.gameMode === 'bot' && this.history.length >= 2) {
            this.history.pop();
            const prev = this.history.pop();
            this.loadFen(prev.fen);
        } else {
            const prev = this.history.pop();
            this.loadFen(prev.fen);
        }
        this.lastMove = null;
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
            for (let char of rows[r]) {
                if (char >= '1' && char <= '8') {
                    col += parseInt(char);
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
        this.halfmove = parseInt(parts[4]) || 0;
        this.fullmove = parseInt(parts[5]) || 1;

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
        const file = coord.charCodeAt(0) - 'a'.charCodeAt(0);
        const rank = 8 - parseInt(coord[1]);
        return rank * 8 + file;
    }

    sqToUci(sq) {
        const file = String.fromCharCode('a'.charCodeAt(0) + (sq % 8));
        const rank = 8 - Math.floor(sq / 8);
        return `${file}${rank}`;
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

            // Highlight last move
            if (this.lastMove && (sq === this.lastMove.from || sq === this.lastMove.to)) {
                sqEl.classList.add('highlight');
            }

            // Highlight selected
            if (this.selectedSquare === sq) {
                sqEl.classList.add('selected');
            }

            // King in check
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

            sqEl.onclick = () => this.handleSquareClick(sq);
            this.boardEl.appendChild(sqEl);
        }
    }

    handleSquareClick(sq) {
        if (this.botThinking) return;
        if (this.gameMode === 'bot' && this.turn !== this.playerSide) return;

        // If clicking on existing selection
        if (this.selectedSquare === sq) {
            this.selectedSquare = null;
            this.legalMovesForSelected = [];
            this.render();
            return;
        }

        // If selecting a destination move
        if (this.selectedSquare !== null) {
            const move = this.legalMovesForSelected.find(m => m.to === sq);
            if (move) {
                // Check pawn promotion
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

        // Selecting own piece
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
        return this.turn === 'w' ? piece === piece.toUpperCase() : piece === piece.toLowerCase();
    }

    executeMove(from, to, promo = null) {
        const piece = this.board[from];
        const target = this.board[to];
        const isCapture = !!target;

        // Save history for undo
        const beforeFen = this.getFen();

        // En passant capture
        if ((piece === 'P' || piece === 'p') && to === this.enpassant) {
            const epPawnSq = this.turn === 'w' ? to + 8 : to - 8;
            this.board[epPawnSq] = null;
        }

        // Move piece
        this.board[to] = promo ? (this.turn === 'w' ? promo.toUpperCase() : promo.toLowerCase()) : piece;
        this.board[from] = null;

        // Castling move rook
        if (piece === 'K' && from === 60) {
            if (to === 62) { this.board[61] = 'R'; this.board[63] = null; }
            if (to === 58) { this.board[59] = 'R'; this.board[56] = null; }
        }
        if (piece === 'k' && from === 4) {
            if (to === 6) { this.board[5] = 'r'; this.board[7] = null; }
            if (to === 2) { this.board[3] = 'r'; this.board[0] = null; }
        }

        // Update castling rights
        if (piece === 'K') { this.castling.K = false; this.castling.Q = false; }
        if (piece === 'k') { this.castling.k = false; this.castling.q = false; }
        if (from === 63 || to === 63) this.castling.K = false;
        if (from === 56 || to === 56) this.castling.Q = false;
        if (from === 7 || to === 7) this.castling.k = false;
        if (from === 0 || to === 0) this.castling.q = false;

        // En passant square
        if (piece === 'P' && from - to === 16) {
            this.enpassant = from - 8;
        } else if (piece === 'p' && to - from === 16) {
            this.enpassant = from + 8;
        } else {
            this.enpassant = null;
        }

        // Halfmove / Fullmove
        if (piece === 'P' || piece === 'p' || isCapture) {
            this.halfmove = 0;
        } else {
            this.halfmove++;
        }
        if (this.turn === 'b') {
            this.fullmove++;
        }

        // Switch turn
        this.turn = this.turn === 'w' ? 'b' : 'w';
        this.lastMove = { from, to };
        this.selectedSquare = null;
        this.legalMovesForSelected = [];

        // Audio
        const inCheck = this.findKingInCheck(this.turn) !== null;
        if (inCheck) {
            audio.playCheck();
        } else if (isCapture) {
            audio.playCapture();
        } else {
            audio.playMove();
        }

        // Save move record
        const uciMove = `${this.sqToUci(from)}${this.sqToUci(to)}${promo || ''}`;
        this.history.push({
            fen: beforeFen,
            uci: uciMove,
            from,
            to,
            piece
        });

        this.fenInputEl.value = this.getFen();
        this.render();
        this.updateHistoryTable();
        this.updateStatus();
        this.triggerAnalysis();

        // Check if next is bot's turn
        if (this.gameMode === 'bot' && this.turn !== this.playerSide) {
            this.triggerBotMove();
        } else if (this.gameMode === 'bvb') {
            setTimeout(() => this.triggerBotMove(), 400);
        }
    }

    triggerAnalysis() {
        if (this.isAnalyzing) return;
        this.isAnalyzing = true;
        const fen = this.getFen();

        fetch('/api/analyze', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ fen, depth: 8 })
        })
        .then(res => res.json())
        .then(data => {
            this.isAnalyzing = false;
            if (data && data.lines) {
                this.updateAnalysisUI(data);
            }
        })
        .catch(() => {
            this.isAnalyzing = false;
        });
    }

    updateAnalysisUI(data) {
        // Update eval bar
        const score = data.score || 0;
        const evalStr = data.eval || "+0.0";
        this.evalBadgeEl.innerText = evalStr;

        // Standard winning probability mapping (0% to 100% white height)
        const winProb = 1.0 / (1.0 + Math.pow(10, -score / 400.0));
        let whitePct = Math.round(winProb * 100);
        whitePct = Math.max(5, Math.min(95, whitePct));

        this.evalWhiteEl.style.height = `${whitePct}%`;

        // Update MultiPV lines
        this.analysisDepthEl.innerText = `Depth ${data.depth || 8}`;
        for (let i = 0; i < 3; i++) {
            const line = data.lines[i];
            if (line) {
                const s = line.score;
                const formattedScore = line.score_type === 'mate' ? `M${s}` : `${s >= 0 ? '+' : ''}${(s / 100).toFixed(1)}`;
                this.pvScores[i].innerText = formattedScore;
                this.pvScores[i].className = `pv-score ${s >= 0 ? 'score-pos' : 'score-neg'}`;

                const moves = line.pv.split(' ');
                this.pvMoves[i].innerText = moves.slice(0, 8).join(' ') + (moves.length > 8 ? ' ...' : '');
                this.pvMoves[i].dataset.firstMove = moves[0];
            } else {
                this.pvScores[i].innerText = "-";
                this.pvMoves[i].innerText = "-";
            }
        }
    }

    triggerBotMove() {
        if (this.botThinking) return;
        this.botThinking = true;
        this.statusTextEl.innerText = "Engine thinking...";

        const fen = this.getFen();
        fetch('/api/bot_move', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ fen, depth: this.botDepth })
        })
        .then(res => res.json())
        .then(data => {
            this.botThinking = false;
            if (data && data.move) {
                const uci = data.move;
                const from = this.uciToSq(uci.substring(0, 2));
                const to = this.uciToSq(uci.substring(2, 4));
                const promo = uci.length >= 5 ? uci[4] : null;
                this.executeMove(from, to, promo);
            }
        })
        .catch(() => {
            this.botThinking = false;
            this.updateStatus();
        });
    }

    findKingInCheck(color) {
        const kingPiece = color === 'w' ? 'K' : 'k';
        const kingSq = this.board.indexOf(kingPiece);
        if (kingSq === -1) return null;
        return this.isSquareAttacked(kingSq, color === 'w' ? 'b' : 'w') ? kingSq : null;
    }

    isSquareAttacked(targetSq, byColor) {
        const tr = Math.floor(targetSq / 8);
        const tc = targetSq % 8;

        for (let sq = 0; sq < 64; sq++) {
            const p = this.board[sq];
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

                if (isRook && (dr === 0 || dc === 0)) {
                    if (this.isRayClear(sq, targetSq)) return true;
                }
                if (isBishop && Math.abs(dr) === Math.abs(dc)) {
                    if (this.isRayClear(sq, targetSq)) return true;
                }
            }
        }
        return false;
    }

    isRayClear(from, to) {
        const r1 = Math.floor(from / 8), c1 = from % 8;
        const r2 = Math.floor(to / 8), c2 = to % 8;
        const stepR = Math.sign(r2 - r1);
        const stepC = Math.sign(c2 - c1);
        let currR = r1 + stepR;
        let currC = c1 + stepC;

        while (currR !== r2 || currC !== c2) {
            if (this.board[currR * 8 + currC] !== null) return false;
            currR += stepR;
            currC += stepC;
        }
        return true;
    }

    generateLegalMoves(from) {
        const moves = [];
        const piece = this.board[from];
        if (!piece) return moves;

        const isWhite = piece === piece.toUpperCase();
        const r = Math.floor(from / 8);
        const c = from % 8;
        const type = piece.toUpperCase();

        const addIfLegal = (to) => {
            const origFrom = this.board[from];
            const origTo = this.board[to];
            this.board[to] = origFrom;
            this.board[from] = null;

            const inCheck = this.findKingInCheck(this.turn) !== null;
            this.board[from] = origFrom;
            this.board[to] = origTo;

            if (!inCheck) {
                moves.push({ from, to });
            }
        };

        // Pawn moves
        if (type === 'P') {
            const dir = isWhite ? -1 : 1;
            const startRank = isWhite ? 6 : 1;
            const oneStep = (r + dir) * 8 + c;

            if (this.board[oneStep] === null) {
                addIfLegal(oneStep);
                const twoStep = (r + 2 * dir) * 8 + c;
                if (r === startRank && this.board[twoStep] === null) {
                    addIfLegal(twoStep);
                }
            }

            // Captures
            for (let dc of [-1, 1]) {
                const targetC = c + dc;
                if (targetC >= 0 && targetC < 8) {
                    const targetSq = (r + dir) * 8 + targetC;
                    const destPiece = this.board[targetSq];
                    if (destPiece && !this.isOwnPiece(destPiece)) {
                        addIfLegal(targetSq);
                    } else if (targetSq === this.enpassant) {
                        addIfLegal(targetSq);
                    }
                }
            }
        }

        // Knight moves
        if (type === 'N') {
            const knightOffsets = [
                [-2, -1], [-2, 1], [-1, -2], [-1, 2],
                [1, -2], [1, 2], [2, -1], [2, 1]
            ];
            for (let [dr, dc] of knightOffsets) {
                const nr = r + dr, nc = c + dc;
                if (nr >= 0 && nr < 8 && nc >= 0 && nc < 8) {
                    const to = nr * 8 + nc;
                    if (!this.board[to] || !this.isOwnPiece(this.board[to])) {
                        addIfLegal(to);
                    }
                }
            }
        }

        // Slider moves (Bishop, Rook, Queen)
        if (type === 'B' || type === 'R' || type === 'Q') {
            const dirs = [];
            if (type === 'B' || type === 'Q') dirs.push([-1, -1], [-1, 1], [1, -1], [1, 1]);
            if (type === 'R' || type === 'Q') dirs.push([-1, 0], [1, 0], [0, -1], [0, 1]);

            for (let [dr, dc] of dirs) {
                let nr = r + dr, nc = c + dc;
                while (nr >= 0 && nr < 8 && nc >= 0 && nc < 8) {
                    const to = nr * 8 + nc;
                    const targetPiece = this.board[to];
                    if (!targetPiece) {
                        addIfLegal(to);
                    } else {
                        if (!this.isOwnPiece(targetPiece)) addIfLegal(to);
                        break;
                    }
                    nr += dr;
                    nc += dc;
                }
            }
        }

        // King moves & Castling
        if (type === 'K') {
            for (let dr = -1; dr <= 1; dr++) {
                for (let dc = -1; dc <= 1; dc++) {
                    if (dr === 0 && dc === 0) continue;
                    const nr = r + dr, nc = c + dc;
                    if (nr >= 0 && nr < 8 && nc >= 0 && nc < 8) {
                        const to = nr * 8 + nc;
                        if (!this.board[to] || !this.isOwnPiece(this.board[to])) {
                            addIfLegal(to);
                        }
                    }
                }
            }

            // Castling
            if (!this.findKingInCheck(this.turn)) {
                if (isWhite && from === 60) {
                    if (this.castling.K && !this.board[61] && !this.board[62] && !this.isSquareAttacked(61, 'b')) {
                        addIfLegal(62);
                    }
                    if (this.castling.Q && !this.board[59] && !this.board[58] && !this.board[57] && !this.isSquareAttacked(59, 'b')) {
                        addIfLegal(58);
                    }
                } else if (!isWhite && from === 4) {
                    if (this.castling.k && !this.board[5] && !this.board[6] && !this.isSquareAttacked(5, 'w')) {
                        addIfLegal(6);
                    }
                    if (this.castling.q && !this.board[3] && !this.board[2] && !this.board[1] && !this.isSquareAttacked(3, 'w')) {
                        addIfLegal(2);
                    }
                }
            }
        }

        return moves;
    }

    updateStatus() {
        const inCheck = this.findKingInCheck(this.turn) !== null;
        let hasLegal = false;
        for (let sq = 0; sq < 64; sq++) {
            if (this.board[sq] && this.isOwnPiece(this.board[sq])) {
                if (this.generateLegalMoves(sq).length > 0) {
                    hasLegal = true;
                    break;
                }
            }
        }

        if (!hasLegal) {
            if (inCheck) {
                this.statusTextEl.innerText = `Checkmate! ${this.turn === 'w' ? 'Black' : 'White'} wins!`;
            } else {
                this.statusTextEl.innerText = "Draw by Stalemate!";
            }
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
            const wMove = this.history[i] ? this.history[i].uci : '';
            const bMove = this.history[i + 1] ? this.history[i + 1].uci : '';

            tr.innerHTML = `
                <td>${moveNum}.</td>
                <td>${wMove}</td>
                <td>${bMove}</td>
            `;
            this.historyBodyEl.appendChild(tr);
        }

        // Scroll to bottom
        const wrapper = document.querySelector('.history-list-wrapper');
        if (wrapper) wrapper.scrollTop = wrapper.scrollHeight;
    }
}

// Bootstrap
window.addEventListener('DOMContentLoaded', () => {
    window.game = new ChessGame();
});
