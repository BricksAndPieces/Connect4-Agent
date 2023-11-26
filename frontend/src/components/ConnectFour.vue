<script setup lang="ts">
import { ref } from 'vue'

let board = Array.from(Array(7), () => Array(6).fill(0))
let heights = Array(7).fill(0)

const currentPlayer = ref(1)
const position = ref('')

const lastCol = ref(-1)
const lastRow = ref(-1)

const play = (col: number) => {
  if (currentPlayer.value !== 1 || getWinner() !== 0 || isDraw()) {
    return
  }

  let valid = dropPiece(col)
  if (valid) {
    agentMove()
  }
}

const dropPiece = (colIndex: number) => {
  if (heights[colIndex] === 6) {
    return false
  }

  console.log('Dropping piece in column ' + colIndex)
  const row = 5 - heights[colIndex]
  board[colIndex][row] = currentPlayer.value
  currentPlayer.value = currentPlayer.value === 1 ? 2 : 1

  position.value += colIndex + 1
  lastCol.value = colIndex
  lastRow.value = row

  heights[colIndex]++
  return true
}

const isDraw = () => heights.every((height) => height === 6)

const agentMove = () => {
  if (currentPlayer.value !== 2 || getWinner() !== 0 || isDraw()) {
    return
  }

  const url = 'http://localhost:8081/api/' + position.value
  const start = new Date().getTime()

  fetch(url)
    .then((response) => response.json())
    .then((data) => {
      console.log(data)
      const end = new Date().getTime()
      const time = Math.min(1000, end - start)

      setTimeout(() => {
        dropPiece(data.col)
      }, 1000 - time)
    })
    .catch((error) => {
      console.error('Error:', error)
      alert('Error: ' + error)
    })
}

const reset = () => {
  board = Array.from(Array(7), () => Array(6).fill(0))
  heights = Array(7).fill(0)
  currentPlayer.value = 1
  position.value = ''
}

const getWinner = () => {
  const tiles = getWonFourTiles()
  return tiles.length !== 0 ? board[tiles[0][0]][tiles[0][1]] : 0
}

const isWinningTile = (player: number, c: number, r: number) => {
  if (getWonFourTiles().length !== 0 || board[c][r] !== 0) {
    return false
  }

  const temp = board.map((arr) => arr.slice())
  temp[c][r] = player

  return getWonFourTiles(temp).length !== 0
}

const getWonFourTiles = (state: number[][] = board) => {
  const checkLine = (startCol: number, startRow: number, colStep: number, rowStep: number) => {
    const player = state[startCol][startRow]
    if (player === 0) {
      return []
    }

    const tiles = []
    for (let i = 0; i < 4; i++) {
      const col = startCol + i * colStep
      const row = startRow + i * rowStep
      if (col > 6 || row > 5 || state[col][row] !== player) {
        return []
      }
      tiles.push([col, row])
    }
    return tiles
  }

  for (let col = 0; col < 7; col++) {
    for (let row = 0; row < 6; row++) {
      const dirs = [
        checkLine(col, row, 1, 0),
        checkLine(col, row, 0, 1),
        checkLine(col, row, 1, 1),
        checkLine(col, row, 1, -1)
      ]

      const tiles = dirs.find((arr) => arr.length > 0)
      if (tiles) {
        return tiles
      }
    }
  }

  return []
}
</script>

<template>
  <div class="container">
    <div class="board">
      <div
        v-for="(col, colIndex) in board"
        :key="colIndex"
        class="col"
        :class="{
          'col-hover': getWinner() === 0 && currentPlayer === 1
        }"
        @click="play(colIndex)"
      >
        <div
          v-for="(tile, rowIndex) in col"
          :key="rowIndex"
          class="tile"
          :style="{
            transition: getWinner() !== 0 ? 'all 1s ease-in-out' : 'none'
          }"
          :class="{
            'tile-1': tile === 1,
            'tile-2': tile === 2,
            'winning-tile-1': isWinningTile(1, colIndex, rowIndex),
            'winning-tile-2': isWinningTile(2, colIndex, rowIndex),
            dull:
              tile !== 0 &&
              getWinner() !== 0 &&
              !getWonFourTiles().some((arr) => arr[0] === colIndex && arr[1] === rowIndex)
          }"
        >
          <div class="last-played" v-if="colIndex == lastCol && rowIndex == lastRow" />
        </div>
      </div>
    </div>

    <div class="button-container" v-if="isDraw() || getWinner() !== 0">
      <button
        class="reset-button"
        :class="{
          p1: getWinner() !== 0 && board[getWonFourTiles()[0][0]][getWonFourTiles()[0][1]] === 1,
          cpu: getWinner() !== 0 && board[getWonFourTiles()[0][0]][getWonFourTiles()[0][1]] === 2
        }"
        @click="reset()"
      >
        <span>Play Again?</span>
      </button>
    </div>

    <!-- hidden div with ref to force updates for 2d array -->
    <div style="display: none">{{ position }}</div>
  </div>
</template>

<style scoped lang="scss">
.container {
  display: flex;
  flex-direction: column;
  height: fit-content;
}

.button-container {
  display: flex;
  justify-content: center;
  margin-top: 20px;

  button {
    margin-top: 10px;
    padding: 10px;
    border-radius: 10px;
    border: none;

    background: linear-gradient(to bottom right, var(--color-p1), var(--color-p2));
    color: #1e1e2e;
    cursor: pointer;

    span {
      font-size: 16px;
      font-weight: bold;
    }
  }

  .p1 {
    background: var(--color-p1);
  }

  .cpu {
    background: var(--color-p2);
  }
}

.board {
  display: flex;
  flex-direction: row;
  border-radius: 46px;
  padding: 10px;
  border: 2px solid var(--color-border);
}

.col {
  display: flex;
  flex-direction: column;
  border-radius: 37px;
}

.col-hover:hover {
  cursor: pointer;

  .tile:not(.tile-1):not(.tile-2):not(.winning-tile-1):not(.winning-tile-2) {
    border: 2px solid var(--color-highlight);
  }
}

.tile {
  width: 70px;
  aspect-ratio: 1/1;
  border: 2px solid var(--color-border);
  margin: 4px;
  border-radius: 35px;

  /* transition only when opacity decreases */
  transition-duration: all 1s ease-in-out;
  transition-delay: 2s;

  display: flex;
  align-items: center;
  justify-content: center;
}

.tile-1 {
  background-color: var(--color-p1);
  border: none;
}

.tile-2 {
  background-color: var(--color-p2);
  border: none;
}
.winning-tile-1 {
  border: 2px solid var(--color-p1);
}

.winning-tile-2 {
  border: 2px solid var(--color-p2);
}

.winning-tile-1.winning-tile-2 {
  border: 2px solid var(--color-p1);
  border-bottom-color: var(--color-p2);
  border-right-color: var(--color-p2);
}

.last-played {
  width: 20px;
  aspect-ratio: 1/1;
  border-radius: 10px;
  background-color: var(--color-background);
}

.dull {
  opacity: 0.3;
}
</style>
