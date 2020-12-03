(ns advent-2020.answers-test
  (:require [clojure.test :refer [deftest is testing]]
            [advent-2020.day-01 :as day-01]
            [advent-2020.day-02 :as day-02]
            [advent-2020.day-03 :as day-03]))

(deftest answers
  (testing "Day 1"
    (is (= day-01/part-1 786811))
    (is (= day-01/part-2 199068980)))

  (testing "Day 2"
    (is (= day-02/part-1 586))
    (is (= day-02/part-2 352)))

  (testing "Day 3"
    (is (= day-03/part-1 156))
    (is (= day-03/part-2 3521829480))))
