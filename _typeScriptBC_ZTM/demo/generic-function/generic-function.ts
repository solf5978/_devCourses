/* eslint-disable */
import { strict as assert } from "assert";

// Generic functions are functions that are designed to work with different
// types of data. They allow you to create a function that can be used with
// various types of data without having to write a separate function for each
// type. This makes your code more efficient, reusable, and easier to maintain.
// Generic functions are especially useful when working with collections of
// data, such as arrays, because they allow you to create a function that can
// work with any type of data in the collection.
//
// Useful links:
// https://www.typescriptlang.org/docs/handbook/2/generics.html#hello-world-of-generics

function getFirstNumber(arr: number[]): number | undefined {
  if (arr.length === 0) {
    return undefined;
  } else {
    return arr[0];
  }
}

function getFirstString(arr: string[]): string | undefined {
  if (arr.length === 0) {
    return undefined;
  } else {
    return arr[0];
  }
}

function getFirstStringOrNumber(
  arr: string[] | number[]
): string | number | undefined {
  return arr[0];
}

function getFirstItem<T>(arr: T[]): T | undefined {
  return arr[0];
}

getFirstNumber([1, 2, 3]); // 1
