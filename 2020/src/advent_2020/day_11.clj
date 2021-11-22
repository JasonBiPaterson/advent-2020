(ns advent-2020.day-11
  (:require [clojure.java.io :as io]
            [clojure.math.numeric-tower :as math]
            [clojure.string :as string]))

(def input
  (->> "advent_2020/day_11/input"
       io/resource
       slurp
       string/split-lines
       (mapv vec)))

#_(def input
  [
   "L.LL.LL.LL"
   "LLLLLLL.LL"
   "L.L.L..L.."
   "LLLL.LL.LL"
   "L.LL.LL.LL"
   "L.LLLLL.LL"
   "..L.L....."
   "LLLLLLLLLL"
   "L.LLLLLL.L"
   "L.LLLLL.LL"
   ])

(def x-count (count input))
(def y-count (count (first input)))

(def seats
  (->> (map-indexed
        (fn [x-idx s]
          (vec
           (map-indexed
            (fn [y-idx char]
              (when (= char \L)
                [x-idx y-idx]))
            s)))
        input)
       (apply concat)
       (filterv some?)))


(defn seat-count [x-idx y-idx coll]
  (->> (for [x (range (dec x-idx) (+ x-idx 2))
             y (range (dec y-idx) (+ y-idx 2))]
         (get-in coll [x y]))
       (filter #{\#})
       count))

(def part-1
  (->> (loop [coll input]
         (let [next-coll (vec
                          (map-indexed
                           (fn [x-idx s]
                             (vec
                              (map-indexed
                               (fn [y-idx char]
                                 (seat-count x-idx y-idx coll)
                                 (case char
                                   \. \.
                                   \L (if (zero? (seat-count x-idx y-idx coll))
                                        \#
                                        \L)
                                   \# (if (<= 5 (seat-count x-idx y-idx coll))
                                        \L
                                        \#)))
                               s)))
                           coll))]
           (if (= next-coll coll)
             coll
             (recur next-coll))))
       (apply concat)
       (filter #{\#})
       count))

(def part-2)
