(ns advent-2020.answers-test
  (:require [clojure.test :refer [deftest is testing]]
            [advent-2020.day-01 :as day-01]
            [advent-2020.day-02 :as day-02]
            [advent-2020.day-03 :as day-03]
            [advent-2020.day-04 :as day-04]
            [advent-2020.day-05 :as day-05]
            [advent-2020.day-06 :as day-06]
            [advent-2020.day-07 :as day-07]
            [advent-2020.day-08 :as day-08]))

(deftest answers
  (testing "Day 1"
    (is (= day-01/part-1 786811))
    (is (= day-01/part-2 199068980)))

  (testing "Day 2"
    (is (= day-02/part-1 586))
    (is (= day-02/part-2 352)))

  (testing "Day 3"
    (is (= day-03/part-1 156))
    (is (= day-03/part-2 3521829480)))

  (testing "Day 4"
    (is (= day-04/part-1 245))
    (is (= day-04/part-2 133)))

  (testing "Day 5"
    (is (= day-05/part-1 919))
    (is (= day-05/part-2 642)))

  (testing "Day 6"
    (is (= day-06/part-1 6161))
    (is (= day-06/part-2 2971)))

  (testing "Day 7"
    (is (= day-07/part-1 287))
    (is (= day-07/part-2 48160)))

  (testing "Day 8"
    (is (= day-08/part-1 1420))
    (is (= day-08/part-2 1245))))
